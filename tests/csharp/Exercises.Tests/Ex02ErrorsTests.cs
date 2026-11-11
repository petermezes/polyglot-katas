using Exercises.Ex02Errors;
using Xunit;

namespace Exercises.Tests.Ex02Errors;

public class ConfigParserTests
{
    private const string ValidConfig = """

        # server settings
        host = tunel.example.no
        port = 8080

        timeout=30
        """;

    [Fact]
    public void Parse_ValidConfig()
    {
        var cfg = ConfigParser.Parse(ValidConfig);

        Assert.Equal(3, cfg.Count);
        Assert.Equal("tunel.example.no", cfg["host"]);
        Assert.Equal("8080", cfg["port"]);
        Assert.Equal("30", cfg["timeout"]);
    }

    [Fact]
    public void Parse_MissingEquals_ThrowsSyntaxException()
    {
        var ex = Assert.Throws<ConfigSyntaxException>(() => ConfigParser.Parse("a = 1\n\nno equals sign"));

        Assert.Equal(3, ex.Line);
        Assert.Equal("no equals sign", ex.Text);
    }

    [Fact]
    public void Parse_EmptyKey_ThrowsSyntaxException()
    {
        var ex = Assert.Throws<ConfigSyntaxException>(() => ConfigParser.Parse("a = 1\n = 2"));

        Assert.Equal(2, ex.Line);
        Assert.Equal("line 2: empty key", ex.Message);
    }

    [Fact]
    public void Parse_DuplicateKey_Throws()
    {
        var ex = Assert.Throws<DuplicateKeyException>(() => ConfigParser.Parse("a = 1\na = 2"));

        Assert.Equal("a", ex.Key);
        Assert.Equal(2, ex.Line);
    }

    [Theory]
    [InlineData("port", true, 8080)]
    [InlineData("host", false, 0)]
    [InlineData("missing", false, 0)]
    public void TryGetInt_DoesNotThrow(string key, bool expectedOk, int expectedValue)
    {
        var cfg = new Dictionary<string, string> { ["port"] = "8080", ["host"] = "x" };

        var ok = ConfigParser.TryGetInt(cfg, key, out var value);

        Assert.Equal(expectedOk, ok);
        Assert.Equal(expectedValue, value);
    }

    [Fact]
    public void GetInt_ThrowsMeaningfulExceptions()
    {
        var cfg = new Dictionary<string, string> { ["port"] = "8080", ["host"] = "x" };

        Assert.Equal(8080, ConfigParser.GetInt(cfg, "port"));
        Assert.Throws<KeyNotFoundException>(() => ConfigParser.GetInt(cfg, "missing"));

        var ex = Assert.Throws<ConfigValueException>(() => ConfigParser.GetInt(cfg, "host"));
        Assert.Equal("host", ex.Key);
        Assert.IsType<FormatException>(ex.InnerException);
    }
}
