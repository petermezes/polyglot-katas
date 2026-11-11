// Exercise 07: builder, an immutable record with init properties, required, with-expression.
namespace Exercises.Ex07Options;

/// <summary>Immutable configuration. Defaults: 30 s, 3 retries, "exercises/1.0", empty headers.</summary>
public sealed record ClientOptions
{
    public required string BaseUrl { get; init; }
    public TimeSpan Timeout { get; init; } = TimeSpan.FromSeconds(30);
    public int Retries { get; init; } = 3;
    public string UserAgent { get; init; } = "exercises/1.0";
    public IReadOnlyDictionary<string, string> Headers { get; init; } = new Dictionary<string, string>();

    /// <summary>
    /// Returns a copy with the timeout multiplied by factor. The original is unchanged.
    /// Tip: this with { Timeout = ... }
    /// </summary>
    public ClientOptions Slower(double factor) => throw new NotImplementedException();
}

/// <summary>Fluent builder: each With* method validates immediately and returns this.</summary>
public sealed class ClientBuilder
{
    // TODO: private fields with default values

    public ClientBuilder(string baseUrl) => throw new NotImplementedException();

    /// <summary>Timeout must be positive, otherwise ArgumentOutOfRangeException.
    /// Tip: ArgumentOutOfRangeException.ThrowIfLessThanOrEqual (.NET 8+).</summary>
    public ClientBuilder WithTimeout(TimeSpan timeout) => throw new NotImplementedException();

    /// <summary>0 to 10, otherwise ArgumentOutOfRangeException.</summary>
    public ClientBuilder WithRetries(int retries) => throw new NotImplementedException();

    public ClientBuilder WithHeader(string key, string value) => throw new NotImplementedException();

    public ClientBuilder WithUserAgent(string userAgent) => throw new NotImplementedException();

    /// <summary>
    /// Creates ClientOptions with an object initializer. BaseUrl must start with "https://",
    /// otherwise ArgumentException. Copy the headers so further builder calls
    /// don't affect options that were already built.
    /// </summary>
    public ClientOptions Build() => throw new NotImplementedException();
}
