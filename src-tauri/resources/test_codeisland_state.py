"""codeisland-state.py 单元测试 — Windows 专属分支校验。"""
import io
import json
import sys
from pathlib import Path

import pytest


SCRIPT = Path(__file__).with_name("codeisland-state.py")


def load_module():
    """以模块方式载入 hook 脚本（脚本文件名含连字符，需手工加载）。"""
    import importlib.util
    spec = importlib.util.spec_from_file_location("codeisland_state", SCRIPT)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def test_pipe_path_constant():
    module = load_module()
    assert module.PIPE_PATH == r"\\.\pipe\codeisland"


def test_no_unix_socket_references():
    source = SCRIPT.read_text(encoding="utf-8")
    assert "AF_UNIX" not in source, "Unix Socket 残留"
    assert "/tmp/codeisland.sock" not in source, "Unix socket 路径残留"
    assert "SOCKET_PATH" not in source, "Unix socket 常量残留"
    assert "_send_via_socket" not in source, "Unix socket 发送函数残留"
    assert "import socket" not in source, "socket 模块 import 残留"


def test_is_server_running_checks_pipe(monkeypatch):
    module = load_module()
    opens: list[tuple[str, str]] = []

    class FakePipe:
        def close(self):
            pass

    def fake_open(path, mode, buffering=0):
        opens.append((path, mode))
        return FakePipe()

    monkeypatch.setattr("builtins.open", fake_open)
    assert module.is_server_running() is True
    assert opens == [(r"\\.\pipe\codeisland", "r+b")]


def test_is_server_running_handles_oserror(monkeypatch):
    module = load_module()

    def fake_open(*args, **kwargs):
        raise OSError("pipe not available")

    monkeypatch.setattr("builtins.open", fake_open)
    assert module.is_server_running() is False


def test_main_exits_when_no_server(monkeypatch):
    module = load_module()
    monkeypatch.setattr(module, "is_server_running", lambda: False)
    monkeypatch.setattr(sys, "stdin", io.StringIO("{}"))
    with pytest.raises(SystemExit) as exc:
        module.main()
    assert exc.value.code == 0


def test_permission_request_allow_path(monkeypatch, capsys):
    module = load_module()
    monkeypatch.setattr(module, "is_server_running", lambda: True)

    sample = {
        "session_id": "sid-1",
        "cwd": "C:\\\\Users\\\\u\\\\proj",
        "hook_event_name": "PermissionRequest",
        "tool_name": "Bash",
        "tool_input": {"command": "dir"},
    }
    monkeypatch.setattr(sys, "stdin", io.StringIO(json.dumps(sample)))

    captured_state: dict = {}

    def fake_send(state):
        captured_state.update(state)
        return {"decision": "allow"}

    monkeypatch.setattr(module, "send_event", fake_send)

    with pytest.raises(SystemExit) as exc:
        module.main()
    assert exc.value.code == 0

    assert captured_state["status"] == "waiting_for_approval"
    assert captured_state["tool"] == "Bash"

    out = capsys.readouterr().out
    payload = json.loads(out)
    assert payload["hookSpecificOutput"]["decision"]["behavior"] == "allow"
