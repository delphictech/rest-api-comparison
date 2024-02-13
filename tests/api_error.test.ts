import { fetchDataFromAPI } from "./utils/helper";

// test used for all error tests
describe("Test API Errors", () => {
  it("should pass if setup is correct", async () => {
    expect(1).toBe(1);
  });

  it("test /coins/alex route ERROR", async () => {
    // check to make sure a 403 error is send if the route or authtoken is incorrect
    await expect(
      fetchDataFromAPI("http://localhost:8000/coins/alex")
    ).rejects.toMatchObject({
      response: { status: 403 },
    });
  });
});
