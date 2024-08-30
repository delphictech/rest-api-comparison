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
app.UseWhen(context => context.Request.Path.StartsWithSegments("/api/resource"),
            appBuilder => appBuilder.UseMiddleware<AuthMiddleware>());

app.MapControllers(); 

app.Run("http://localhost:8000");
