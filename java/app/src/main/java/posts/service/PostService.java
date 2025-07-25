package posts.service;

import org.springframework.stereotype.Service;
import org.springframework.validation.annotation.Validated;
import posts.dto.PostInput;
import posts.dto.PostResponse;
import posts.model.Post;

import javax.validation.constraints.NotBlank;
import javax.validation.constraints.NotNull;
import java.util.ArrayList;
import java.util.Collections;
import java.util.List;
import java.util.UUID;
import java.util.stream.Collectors;

@Service
@Validated
public class PostService {

    private final List<Post> posts;

    public PostService() {
        this.posts = Collections.synchronizedList(new ArrayList<>());
    }

    /**
     * @return A {@link List} of all the Posts.
     */
    public List<PostResponse> getAll() {
        return posts.stream().map(this::mapToResponse).collect(Collectors.toList());
    }

    /**
     * @param id The id of the post to get.
     * @return An instance of {@link PostResponse} with the corresponding id.
     */
    public PostResponse getById(@NotBlank final String id) {
        return posts.stream()
                .filter(post -> post.getId().equals(id)).map(this::mapToResponse)
                .findFirst()
                .orElseThrow(() -> new IllegalArgumentException("No post with id " + id));
    }

    /**
     * Create a new Post from the given input.
     *
     * @param input An instance of {@link PostInput} from which to craft the Post from.
     * @return The freshly created Post.
     */
    public PostResponse create(@NotNull final PostInput input) {
        Post newPost = Post.builder()
                .id(UUID.randomUUID().toString())
                .author(input.getAuthor())
                .date(input.getDate())
                .content(input.getContent())
                .build();

        synchronized (posts) { posts.add(newPost); }
        return mapToResponse(newPost);
    }

    /**
     * Update a Post from the given input.
     *
     * @param id    The id of the Post to update.
     * @param input An instance of {@link PostInput} from which to update the Post from.
     * @return The updated Post.
     * <p>
     * @throws IllegalArgumentException If no post with the given id exists.
     */
    public PostResponse update(@NotBlank final String id, @NotNull final PostInput input) {
        Post postToUpdate = findPostById(id);

        postToUpdate.setAuthor(input.getAuthor());
        postToUpdate.setDate(input.getDate());
        postToUpdate.setContent(input.getContent());

        return mapToResponse(postToUpdate);
    }

    /**
     * Delete the Post with the given id.
     * @param id The id of the post to delete.
     *
     * @throws IllegalArgumentException if no Post with the given id exists.
     */
    public void delete(String id) {
        Post postToDelete = findPostById(id);
        synchronized (posts) { posts.remove(postToDelete); }
    }


    private Post findPostById(@NotBlank final String id) {
        synchronized (posts) {
            return posts.stream()
                    .filter(post -> post.getId().equals(id))
                    .findFirst()
                    .orElseThrow(() -> new IllegalArgumentException("No post with id " + id));
        }
    }

    private PostResponse mapToResponse(Post entity) {
        PostResponse response = new PostResponse();

        response.setId(entity.getId());
        response.setAuthor(entity.getAuthor());
        response.setDate(entity.getDate());
        response.setContent(entity.getContent());

        return response;
    }
}
