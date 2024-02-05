import { fetchDataFromAPI } from "./utils/helper";

// test used for checking all successful api routes
describe("Test Successful API Routes", () => {

  it("should pass if setup is correct", async () => {
    expect(1).toBe(1);
  });

  // it("test /coins/alex route SUCCESS", async () => {
  //   const result = await fetchDataFromAPI("http://localhost:8000/coins/alex", {
  //     authtoken: "123",
  //   });
  //   // Check if the result matches the expected data
  //   expect(result.data.balance).toEqual(100);
  // });
});
