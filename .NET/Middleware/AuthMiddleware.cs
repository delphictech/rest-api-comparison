using MyProject.Data;

public class AuthMiddleware
{
    public async Task InvokeAsync(HttpContext context)
    {
        var userID = context.Request.RouteValues["userID"]?.ToString();
        var authToken = context.Request.Headers["authtoken"].FirstOrDefault();

        if (userID != null && authToken != null && MockData.MockLoginDetails.ContainsKey(userID))
        {
            var user = MockData.MockLoginDetails[userID];
            if (user.AuthToken == authToken)
            {
                await _next(context);
                return;
            }
        }

        context.Response.StatusCode = StatusCodes.Status403Forbidden;
        await context.Response.WriteAsync("NOT AUTHORIZED");
    }
}