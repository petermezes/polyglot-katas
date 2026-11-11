// Exercise 02: errors — custom exceptions, InnerException, the TryParse pattern.
namespace Exercises.Ex02Errors;

/// <summary>A line without '=' or with an empty key.</summary>
public class ConfigSyntaxException : Exception
{
    public int Line { get; }
    public string Text { get; }

    public ConfigSyntaxException(int line, string text, string reason)
        : base($"line {line}: {reason}")
    {
        Line = line;
        Text = text;
    }
}

public class DuplicateKeyException : Exception
{
    public string Key { get; }
    public int Line { get; }

    public DuplicateKeyException(string key, int line)
        : base($"line {line}: duplicate key '{key}'")
    {
        Key = key;
        Line = line;
    }
}

/// <summary>The value cannot be converted. The original exception is in InnerException (like %w in Go).</summary>
public class ConfigValueException : Exception
{
    public string Key { get; }

    public ConfigValueException(string key, Exception inner)
        : base($"key '{key}': invalid value", inner)
    {
        Key = key;
    }
}

public static class ConfigParser
{
    /// <summary>
    /// Parses "key = value" line by line. Empty lines and lines starting with '#' are ignored.
    /// </summary>
    public static Dictionary<string, string> Parse(string text)
    {
        var cfg = new Dictionary<string, string>();
        var lines = text.Split('\n');
        for (var i = 0; i < lines.Length; i++)
        {
            var lineNo = i + 1;
            var line = lines[i].Trim();
            if (line.Length == 0 || line.StartsWith('#'))
                continue;

            var eq = line.IndexOf('=');
            if (eq < 0)
                throw new ConfigSyntaxException(lineNo, line, "missing '='");

            var key = line[..eq].Trim();
            var value = line[(eq + 1)..].Trim();
            if (key.Length == 0)
                throw new ConfigSyntaxException(lineNo, line, "empty key");

            if (!cfg.TryAdd(key, value))
                throw new DuplicateKeyException(key, lineNo);
        }
        return cfg;
    }

    /// <summary>TryParse pattern: no exceptions, the result goes through an out parameter.</summary>
    public static bool TryGetInt(IReadOnlyDictionary<string, string> cfg, string key, out int value)
    {
        value = 0;
        return cfg.TryGetValue(key, out var s) && int.TryParse(s, out value);
    }

    /// <summary>
    /// Missing key: KeyNotFoundException. Invalid number: ConfigValueException
    /// with the original FormatException as InnerException.
    /// </summary>
    public static int GetInt(IReadOnlyDictionary<string, string> cfg, string key)
    {
        if (!cfg.TryGetValue(key, out var s))
            throw new KeyNotFoundException($"missing key '{key}'");
        try
        {
            return int.Parse(s);
        }
        catch (FormatException ex)
        {
            throw new ConfigValueException(key, ex);
        }
    }
}
