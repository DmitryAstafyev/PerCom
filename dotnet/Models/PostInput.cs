namespace DotNetServer.Models;

public class PostInput
{
    public string Author { get; set; } = default!;          
    public DateTime Date { get; set; }                      
    public string Content { get; set; } = default!;         
}
