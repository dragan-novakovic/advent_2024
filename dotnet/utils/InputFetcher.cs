using System.Text.Json;

namespace Advent.Utils
{
    public static class InputFetcher
    {
        private static readonly HttpClient client = new();

        public static async Task<T> FetchDataAsync<T>(string url)
        {
            try
            {
                HttpResponseMessage response = await client.GetAsync(url);
                response.EnsureSuccessStatusCode();
                string responseBody = await response.Content.ReadAsStringAsync();
                return JsonSerializer.Deserialize<T>(responseBody)!;
            }
            catch (HttpRequestException e)
            {
                Console.WriteLine($"Request error: {e.Message}");
                return default!;
            }
        }
    }
}