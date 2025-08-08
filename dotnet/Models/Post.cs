namespace DotNetServer.Models;

public class Post
{
    public string Id { get; set; } = default!;             
    public string Author { get; set; } = default!;         
    public DateTime Date { get; set; }                     
    public string Content { get; set; } = default!;        
}
