import axios from "axios";

// Function that makes an API call to localhost:8000
async function fetchDataFromAPI(URL: string, headers?: Record<string, string>) {
  try {
    const response = await axios.get(URL, { headers });
    return response.data;
  } catch (error) {
    throw error;
  }
}

describe("Test Successful API Routes", () => {
  it("should pass if setup is correct", async () => {
    expect(1).toBe(1);
  });

  it("test default route data", async () => {
    // Call the function that fetches data from the API
    const result = await fetchDataFromAPI("http://localhost:8000");

    // Check if the result matches the expected data
    expect(result).toEqual("HELLO WORLD");
  });

  it("test the /test route", async () => {
    // Mocking the API response
    const mockData = { message: "testing route", code: 200 };

    // Call the function that fetches data from the API
    const result = await fetchDataFromAPI("http://localhost:8000/test");

    // Check if the result matches the expected data
    expect(result).toEqual(mockData);
  });

  it("test /coins/alex route SUCCESS", async () => {
    // Call the function that fetches data from the API
    const result = await fetchDataFromAPI("http://localhost:8000/coins/alex", {
      authtoken: "123",
    });

    console.log("results", result);

    // Check if the result matches the expected data
    expect(result.data.balance).toEqual(100);
  });
});

describe("Test API Errors", () => {
  it("test /coins/alex route ERROR", async () => {
    await expect(
      fetchDataFromAPI("http://localhost:8000/coins/alex")
    ).rejects.toMatchObject({
      response: { status: 403 },
    });
  });
});
