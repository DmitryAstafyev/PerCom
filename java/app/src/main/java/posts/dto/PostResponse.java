package posts.dto;

import lombok.Getter;
import lombok.Setter;

import java.time.ZonedDateTime;

@Getter
@Setter
public class PostResponse {

    private String id;
    private String author;
    private ZonedDateTime date;
    private String content;

}
