// Exercise 02: errors — custom exceptions, InnerException, the TryParse pattern.
namespace Exercises.Ex02Errors;

/// <summary>
/// A line without '=' or with an empty key. Message has the form "line {line}: {reason}",
/// e.g. "line 2: empty key".
/// </summary>
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

/// <summary>TODO: Key and Line properties, constructor (string key, int line), a meaningful Message.</summary>
public class DuplicateKeyException : Exception
{
    public string Key => throw new NotImplementedException();
    public int Line => throw new NotImplementedException();
}

/// <summary>
/// The value cannot be converted. The original exception must be in InnerException (like %w in Go).
/// TODO: Key property, constructor (string key, Exception inner) calling base(message, inner).
/// </summary>
public class ConfigValueException : Exception
{
    public string Key => throw new NotImplementedException();
}

public static class ConfigParser
{
    /// <summary>
    /// Parses "key = value" line by line (numbered from 1). Empty lines and lines
    /// starting with '#' are ignored. Both key and value are trimmed.
    ///   line without '=' -> ConfigSyntaxException(line, trimmedLine, "missing '='")
    ///   empty key        -> ConfigSyntaxException(line, trimmedLine, "empty key")
    ///   duplicate key    -> DuplicateKeyException
    /// Tip: Split('\n'), Trim, IndexOf('='), the range operator line[..eq], Dictionary.TryAdd.
    /// </summary>
    public static Dictionary<string, string> Parse(string text) =>
        throw new NotImplementedException();

    /// <summary>
    /// TryParse pattern: no exceptions, the result goes through an out parameter (0 on failure).
    /// Tip: TryGetValue and int.TryParse can be combined with &amp;&amp;.
    /// </summary>
    public static bool TryGetInt(IReadOnlyDictionary<string, string> cfg, string key, out int value) =>
        throw new NotImplementedException();

    /// <summary>
    /// Missing key: KeyNotFoundException. Invalid number: ConfigValueException
    /// with the original FormatException from int.Parse as InnerException.
    /// </summary>
    public static int GetInt(IReadOnlyDictionary<string, string> cfg, string key) =>
        throw new NotImplementedException();
}
