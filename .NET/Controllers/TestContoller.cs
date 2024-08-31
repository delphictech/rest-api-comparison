using Microsoft.AspNetCore.Mvc;
using MyProject.Data;

namespace TestController.Controllers
{
    [ApiController]
    [Route("/")]
    public class TestController : ControllerBase
    {
        [HttpGet]
        public IActionResult GetRoot()
        {
            return Ok("HELLO WORLD");
        }

        [HttpGet]
        [Route("test")]
        public IActionResult GetTest()
        {
            var response = new
            {
                message = "testing route",
                code = 200
            };

            return Ok(response);
        }

        [HttpGet]
        [Route("coins/{userId}")]
        public IActionResult GetUserCoins(string userId)
        {
            // Default balance to 0
            int balance = 0;

            // Check if the userId exists in MockCoinDetails
            if (MockData.MockCoinDetails.ContainsKey(userId))
            {
                balance = MockData.MockCoinDetails[userId].Balance;
            }

            var response = new
            {
                data = new
                {
                    userId = userId,
                    message = $"Coins for user {userId}",
                    code = 200,
                    balance = balance
                }
            };

            return Ok(response);  // Correctly return the response
    } // Make sure this closing brace exists for the method

} // Make sure this closing brace exists for the class
} // Make sure this closing brace exists for the namespace
