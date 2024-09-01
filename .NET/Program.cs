var builder = WebApplication.CreateBuilder(args);

// Add services to the container.
builder.Services.AddControllers(); // Add controller services
builder.Services.AddEndpointsApiExplorer();
builder.Services.AddSwaggerGen();

var app = builder.Build();

// Configure the HTTP request pipeline.
if (app.Environment.IsDevelopment())
{
    app.UseSwagger();
    app.UseSwaggerUI();
}

app.UseHttpsRedirection();

app.UseAuthorization();

// Apply middleware to specific paths
app.UseWhen(context =>
{
    var path = context.Request.Path.Value;

    // Check if the path matches /coins/{userId} with some validation
    return path != null && path.StartsWith("/coins/") && path.Split('/').Length == 3;
},
appBuilder => appBuilder.UseMiddleware<AuthMiddleware>());

app.MapControllers(); 

app.Run("http://localhost:8000");
