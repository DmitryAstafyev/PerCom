using Microsoft.AspNetCore.Mvc;
using System.Collections.Concurrent;
using System.ComponentModel.DataAnnotations;
using DotNetServer.Models;

namespace DotNetServer.Controllers;

[ApiController]
[Route("posts")]
public class PostsController : ControllerBase
{
    private static readonly ConcurrentDictionary<string, Post> _posts = new();

    // GET /posts
    [HttpGet]
    public IActionResult GetAll()
    {
        return Ok(_posts.Values);
    }

    // GET /posts/{postId}
    [HttpGet("{postId}")]
    public IActionResult GetById(string postId)
    {
        if (_posts.TryGetValue(postId, out var post))
            return Ok(post);

        return NotFound();
    }

    // POST /posts
    [HttpPost]
    public IActionResult Create([FromBody] PostInput input)
    {
        var id = Guid.NewGuid().ToString();
        var post = new Post
        {
            Id = id,
            Author = input.Author,
            Date = input.Date,
            Content = input.Content
        };

        _posts.TryAdd(id, post);
        return CreatedAtAction(nameof(GetById), new { postId = id }, post);
    }

    // PUT /posts/{postId}
    [HttpPut("{postId}")]
    public IActionResult Update(string postId, [FromBody] PostInput input)
    {
        if (!_posts.ContainsKey(postId))
            return NotFound();

        var post = new Post
        {
            Id = postId,
            Author = input.Author,
            Date = input.Date,
            Content = input.Content
        };

        _posts[postId] = post;
        return Ok(post);
    }

    // DELETE /posts/{postId}
    [HttpDelete("{postId}")]
    public IActionResult Delete(string postId)
    {
        if (_posts.TryRemove(postId, out _))
            return NoContent();

        return NotFound();
    }
}
