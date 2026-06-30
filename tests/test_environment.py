from rlbot_flatbuffers import (
    CustomBot,
    EnvironmentVariable,
    ScriptConfiguration,
)


class TestCustomBotEnvironment:
    def test_default_is_none(self):
        bot = CustomBot(name="n", root_dir="r", run_command="c", agent_id="a")
        assert bot.environment is None

    def test_none_roundtrip(self):
        bot = CustomBot(name="n", root_dir="r", run_command="c", agent_id="a")
        bot2 = CustomBot.unpack(bot.pack())
        assert bot2.environment is None

    def test_populated_fields(self):
        envs = [
            EnvironmentVariable(name="FOO", value="bar"),
            EnvironmentVariable(name="BAZ", value="qux"),
        ]
        bot = CustomBot(
            name="n", root_dir="r", run_command="c", agent_id="a", environment=envs
        )
        assert bot.environment is not None
        assert len(bot.environment) == 2
        assert bot.environment[0].name == "FOO"
        assert bot.environment[0].value == "bar"
        assert bot.environment[1].name == "BAZ"
        assert bot.environment[1].value == "qux"

    def test_populated_roundtrip(self):
        envs = [
            EnvironmentVariable(name="FOO", value="bar"),
            EnvironmentVariable(name="BAZ", value="qux"),
        ]
        bot = CustomBot(
            name="n", root_dir="r", run_command="c", agent_id="a", environment=envs
        )
        bot2 = CustomBot.unpack(bot.pack())
        assert bot2.environment is not None
        assert len(bot2.environment) == 2
        assert bot2.environment[0].name == "FOO"
        assert bot2.environment[0].value == "bar"
        assert bot2.environment[1].name == "BAZ"
        assert bot2.environment[1].value == "qux"

    def test_repr_with_values(self):
        envs = [
            EnvironmentVariable(name="FOO", value="bar"),
            EnvironmentVariable(name="BAZ", value="qux"),
        ]
        bot = CustomBot(
            name="n", root_dir="r", run_command="c", agent_id="a", environment=envs
        )
        r = repr(bot)
        assert "FOO" in r
        assert "BAZ" in r
        assert "bar" in r
        assert "environment=[" in r

    def test_repr_with_none(self):
        bot = CustomBot(name="n", root_dir="r", run_command="c", agent_id="a")
        assert "environment=None" in repr(bot)


class TestScriptConfigurationEnvironment:
    def test_default_is_none(self):
        script = ScriptConfiguration(
            name="s", root_dir="r", run_command="c", agent_id="a"
        )
        assert script.environment is None

    def test_none_roundtrip(self):
        script = ScriptConfiguration(
            name="s", root_dir="r", run_command="c", agent_id="a"
        )
        script2 = ScriptConfiguration.unpack(script.pack())
        assert script2.environment is None

    def test_populated_fields(self):
        envs = [EnvironmentVariable(name="PATH", value="/usr/bin")]
        script = ScriptConfiguration(
            name="s", root_dir="r", run_command="c", agent_id="a", environment=envs
        )
        assert script.environment is not None
        assert len(script.environment) == 1
        assert script.environment[0].name == "PATH"
        assert script.environment[0].value == "/usr/bin"

    def test_populated_roundtrip(self):
        envs = [EnvironmentVariable(name="PATH", value="/usr/bin")]
        script = ScriptConfiguration(
            name="s", root_dir="r", run_command="c", agent_id="a", environment=envs
        )
        script2 = ScriptConfiguration.unpack(script.pack())
        assert script2.environment is not None
        assert len(script2.environment) == 1
        assert script2.environment[0].name == "PATH"
        assert script2.environment[0].value == "/usr/bin"

    def test_repr_with_values(self):
        envs = [EnvironmentVariable(name="PATH", value="/usr/bin")]
        script = ScriptConfiguration(
            name="s", root_dir="r", run_command="c", agent_id="a", environment=envs
        )
        r = repr(script)
        assert "PATH" in r
        assert "/usr/bin" in r
        assert "environment=[" in r

    def test_repr_with_none(self):
        script = ScriptConfiguration(
            name="s", root_dir="r", run_command="c", agent_id="a"
        )
        assert "environment=None" in repr(script)
