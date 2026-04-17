#!/usr/bin/env python3
"""Code Island Hook (Windows 专属)。

通过 Named Pipe `\\\\.\\pipe\\codeisland` 与 Tauri 后端通信；PermissionRequest 时
阻塞等待后端决策，其他事件只上报。"""
import json
import os
import sys

TIMEOUT_SECONDS = 300
PIPE_PATH = r"\\.\pipe\codeisland"


def is_server_running() -> bool:
    """探测 Code Island Named Pipe 是否可连接。"""
    try:
        handle = open(PIPE_PATH, "r+b", buffering=0)
        handle.close()
        return True
    except OSError:
        return False


def send_event(state):
    """向 Named Pipe 写入事件，PermissionRequest 场景同步等待后端回复。"""
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
