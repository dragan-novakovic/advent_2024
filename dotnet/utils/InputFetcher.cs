namespace Advent.Utils
{
    public static class InputFetcher
    {
        private static readonly HttpClient client = new();

        public static async Task<string> FetchDataAsync(string url)
        {
            try
            {
                HttpResponseMessage response = await client.GetAsync(url);
                response.EnsureSuccessStatusCode();
                string responseBody = await response.Content.ReadAsStringAsync();
                return responseBody;
            }
            catch (HttpRequestException e)
            {
                Console.WriteLine($"Request error: {e.Message}");
                return null;
            }
        }
    }
}