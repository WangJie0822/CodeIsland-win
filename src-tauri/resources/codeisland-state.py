#!/usr/bin/env python3
"""
Code Island Hook (Windows + macOS)
- Windows: Named Pipe
- macOS/Linux: Unix Socket (/tmp/codeisland.sock)
- PermissionRequest 时阻塞等待用户决策
"""
import json
import os
import platform
import sys

TIMEOUT_SECONDS = 300
IS_WINDOWS = platform.system() == "Windows"
PIPE_PATH = r"\\.\pipe\codeisland"
SOCKET_PATH = "/tmp/codeisland.sock"


def is_server_running():
    """探测 Code Island IPC 服务是否可达"""
    if IS_WINDOWS:
        try:
            handle = open(PIPE_PATH, "r+b", buffering=0)
            handle.close()
            return True
        except OSError:
            return False
    else:
        return os.path.exists(SOCKET_PATH)


def send_event(state):
    if IS_WINDOWS:
        return _send_via_pipe(state)
    else:
        return _send_via_socket(state)


def _send_via_pipe(state):
    try:
        pipe = open(PIPE_PATH, "r+b", buffering=0)
        pipe.write(json.dumps(state).encode("utf-8"))
        pipe.flush()

        if state.get("status") == "waiting_for_approval":
            import time
            start = time.time()
            data = b""
            while time.time() - start < TIMEOUT_SECONDS:
                try:
                    chunk = pipe.read(4096)
                    if chunk:
                        data += chunk
                        break
                except Exception:
                    time.sleep(0.1)
                    continue
            pipe.close()
            if data:
                return json.loads(data.decode("utf-8"))
        else:
            try:
                pipe.read(64)
            except Exception:
                pass
            pipe.close()
        return None
    except (OSError, json.JSONDecodeError):
        return None


def _send_via_socket(state):
    import socket
    try:
        sock = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
        sock.settimeout(TIMEOUT_SECONDS)
        sock.connect(SOCKET_PATH)
        sock.sendall(json.dumps(state).encode("utf-8"))

        if state.get("status") == "waiting_for_approval":
            response = sock.recv(4096)
            sock.close()
            if response:
                return json.loads(response.decode("utf-8"))
        else:
            sock.close()
        return None
    except (socket.error, OSError, json.JSONDecodeError):
        return None


def main():
    if not is_server_running():
        sys.exit(0)

    try:
        data = json.load(sys.stdin)
    except json.JSONDecodeError:
        sys.exit(1)

    session_id = data.get("session_id", "unknown")
    event = data.get("hook_event_name", "")
    cwd = data.get("cwd", "")
    tool_input = data.get("tool_input", {})
    claude_pid = os.getppid()

    state = {
        "session_id": session_id,
        "cwd": cwd,
        "event": event,
        "pid": claude_pid,
        "tty": None,
    }

    if event == "UserPromptSubmit":
        state["status"] = "processing"

    elif event == "PreToolUse":
        state["status"] = "running_tool"
        state["tool"] = data.get("tool_name")
        state["tool_input"] = tool_input
        tool_use_id = data.get("tool_use_id")
        if tool_use_id:
            state["tool_use_id"] = tool_use_id

    elif event == "PostToolUse":
        state["status"] = "processing"
        state["tool"] = data.get("tool_name")
        state["tool_input"] = tool_input
        tool_use_id = data.get("tool_use_id")
        if tool_use_id:
            state["tool_use_id"] = tool_use_id

    elif event == "PermissionRequest":
        state["status"] = "waiting_for_approval"
        state["tool"] = data.get("tool_name")
        state["tool_input"] = tool_input

        response = send_event(state)

        if response:
            decision = response.get("decision", "ask")
            reason = response.get("reason", "")

            if decision == "allow":
                output = {
                    "hookSpecificOutput": {
                        "hookEventName": "PermissionRequest",
                        "decision": {"behavior": "allow"},
                    }
                }
                print(json.dumps(output))
                sys.exit(0)
            elif decision == "deny":
                output = {
                    "hookSpecificOutput": {
                        "hookEventName": "PermissionRequest",
                        "decision": {
                            "behavior": "deny",
                            "message": reason or "Denied by user via CodeIsland",
                        },
                    }
                }
                print(json.dumps(output))
                sys.exit(0)
        sys.exit(0)

    elif event == "Notification":
        notification_type = data.get("notification_type")
        if notification_type == "permission_prompt":
            sys.exit(0)
        elif notification_type == "idle_prompt":
            state["status"] = "waiting_for_input"
        else:
            state["status"] = "notification"
        state["notification_type"] = notification_type
        state["message"] = data.get("message")

    elif event == "Stop":
        state["status"] = "waiting_for_input"
    elif event == "SubagentStop":
        state["status"] = "waiting_for_input"
    elif event == "SessionStart":
        state["status"] = "waiting_for_input"
    elif event == "SessionEnd":
        state["status"] = "ended"
    elif event == "PreCompact":
        state["status"] = "compacting"
    else:
        state["status"] = "unknown"

    send_event(state)


if __name__ == "__main__":
    main()
