// Exercise 07: builder, an immutable record with init properties, required, with-expression.
namespace Exercises.Ex07Options;

public sealed record ClientOptions
{
    public required string BaseUrl { get; init; }
    public TimeSpan Timeout { get; init; } = TimeSpan.FromSeconds(30);
    public int Retries { get; init; } = 3;
    public string UserAgent { get; init; } = "exercises/1.0";
    public IReadOnlyDictionary<string, string> Headers { get; init; } = new Dictionary<string, string>();

    /// <summary>Returns a copy with the timeout multiplied by factor (with-expression). The original is unchanged.</summary>
    public ClientOptions Slower(double factor) => this with { Timeout = Timeout * factor };
}

/// <summary>Fluent builder: each method returns this and validates immediately.</summary>
public sealed class ClientBuilder
{
    private readonly string _baseUrl;
    private readonly Dictionary<string, string> _headers = new();
    private TimeSpan _timeout = TimeSpan.FromSeconds(30);
    private int _retries = 3;
    private string _userAgent = "exercises/1.0";

    public ClientBuilder(string baseUrl) => _baseUrl = baseUrl;

    public ClientBuilder WithTimeout(TimeSpan timeout)
    {
        ArgumentOutOfRangeException.ThrowIfLessThanOrEqual(timeout, TimeSpan.Zero);
        _timeout = timeout;
        return this;
    }

    public ClientBuilder WithRetries(int retries)
    {
        ArgumentOutOfRangeException.ThrowIfNegative(retries);
        ArgumentOutOfRangeException.ThrowIfGreaterThan(retries, 10);
        _retries = retries;
        return this;
    }

    public ClientBuilder WithHeader(string key, string value)
    {
        _headers[key] = value;
        return this;
    }

    public ClientBuilder WithUserAgent(string userAgent)
    {
        _userAgent = userAgent;
        return this;
    }

    /// <summary>
    /// Creates ClientOptions. BaseUrl must start with "https://", otherwise ArgumentException.
    /// Copies the headers so further builder calls don't affect options that were already built.
    /// </summary>
    public ClientOptions Build()
    {
        if (!_baseUrl.StartsWith("https://", StringComparison.Ordinal))
            throw new ArgumentException($"URL must start with https://: {_baseUrl}");

        return new ClientOptions
        {
            BaseUrl = _baseUrl,
            Timeout = _timeout,
            Retries = _retries,
            UserAgent = _userAgent,
            Headers = new Dictionary<string, string>(_headers),
        };
    }
}
