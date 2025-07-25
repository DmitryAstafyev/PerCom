package posts.controller;

import lombok.RequiredArgsConstructor;
import org.springframework.http.HttpStatus;
import org.springframework.http.ResponseEntity;
import org.springframework.validation.annotation.Validated;
import org.springframework.web.bind.annotation.*;
import posts.dto.PostInput;
import posts.dto.PostResponse;
import posts.service.PostService;

import javax.validation.Valid;
import javax.validation.constraints.NotBlank;
import java.util.List;

@Validated
@RestController
@RequestMapping("/posts")

@RequiredArgsConstructor
public class PostController {

    private final PostService postService;

    @GetMapping
    public ResponseEntity<List<PostResponse>> getAllPosts() {
        return ResponseEntity.ok(postService.getAll());
    }

    @GetMapping("/{id}")
    public ResponseEntity<PostResponse> getPostById(@NotBlank final @PathVariable String id) {
        return ResponseEntity.ok(postService.getById(id));
    }

    @PostMapping
    public ResponseEntity<PostResponse> createPost(@Valid @RequestBody PostInput input) {
        return ResponseEntity.status(HttpStatus.CREATED).body(postService.create(input));
    }

    @PutMapping("/{id}")
    public ResponseEntity<PostResponse> updatePost(@PathVariable String id, @Valid @RequestBody PostInput input) {
        return ResponseEntity.ok(postService.update(id, input));
    }

    @DeleteMapping("/{id}")
    public ResponseEntity<Void> deletePost(@NotBlank final @PathVariable String id) {
        postService.delete(id);
        return ResponseEntity.noContent().build();
    }
}
