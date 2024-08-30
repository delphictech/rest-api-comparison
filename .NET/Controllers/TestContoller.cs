using Microsoft.AspNetCore.Mvc;

namespace TestController.Controllers
{
    [ApiController]
    [Route("/")] // Base route for the controller, applied to the root route "/"
    public class TestController : ControllerBase
    {
        // Route: /
        [HttpGet]
        public IActionResult GetRoot()
        {
            return Ok("HELLO WORLD");
        }

        // Route: /test
        [HttpGet]
        [Route("test")]
        public IActionResult GetTest()
        {
            var response = new
            {
                message = "test route",
                code = 200
            };

            return Ok(response);
        }

          // Route: /coins/{userId}
        [HttpGet]
        [Route("coins/{userId}")]
        public IActionResult GetUserCoins(string userId)
        {
            // Now you can use the userId parameter in your method
            var response = new
            {
                userId = userId,
                message = $"Coins for user {userId}",
                code = 200
            };

            return Ok(response);
        }
        
    }
}
