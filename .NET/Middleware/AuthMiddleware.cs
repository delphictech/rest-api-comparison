using Microsoft.AspNetCore.Http;
using MyProject.Data;
using System.Threading.Tasks;

public class AuthMiddleware
{
    private readonly RequestDelegate _next;  // Step 1: Declare the _next field

    // Step 2: Constructor accepting RequestDelegate
    public AuthMiddleware(RequestDelegate next)
    {
        _next = next;
    }

    // Step 3: Correctly reference _next in InvokeAsync
    public async Task InvokeAsync(HttpContext context)
    {
        var userID = context.Request.RouteValues["userID"]?.ToString();
        var authToken = context.Request.Headers["authtoken"].FirstOrDefault();

        // Log the incoming userID and authToken for debugging purposes
        Console.WriteLine($"UserID: {userID}, AuthToken: {authToken}");

        if (userID != null && authToken != null && MockData.MockLoginDetails.ContainsKey(userID))
        {
            var user = MockData.MockLoginDetails[userID];
            if (user.AuthToken == authToken)
            {
                await _next(context);  // Correctly using _next to invoke the next middleware in the pipeline
                return;
            }
        }

        context.Response.StatusCode = StatusCodes.Status403Forbidden;
        await context.Response.WriteAsync("NOT AUTHORIZED");
    }
}
