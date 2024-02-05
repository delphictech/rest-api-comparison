import { fetchDataFromAPI } from "./utils/helper";

// test used for checking all successful api routes
describe("Test Successful API Routes", () => {
  // test jest setup
  it("should pass if setup is correct", async () => {
    expect(1).toBe(1);
  });

  it("test default route data", async () => {
    // Call the function that fetches data from the API
    const result = await fetchDataFromAPI("http://localhost:8000/");

    console.log("results", result);

    // Check if the result matches the expected data
    expect(result).toEqual("HELLO WORLD");
  });

  // it("test the /test route", async () => {
  //   // Mocking the API response
  //   const mockData = { message: "testing route", code: 200 };

  //   const result = await fetchDataFromAPI("http://localhost:8000/test");

  //   console.log("results", result);

  //   expect(result).toEqual(mockData);
  // });
});
