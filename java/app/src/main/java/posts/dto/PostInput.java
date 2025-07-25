package posts.dto;


import lombok.Getter;
import lombok.Setter;

import javax.validation.constraints.NotBlank;
import javax.validation.constraints.NotNull;
import java.time.ZonedDateTime;

@Getter
@Setter
public class PostInput {

    @NotBlank(message = "Author is required")
    private String author;

    @NotNull(message = "Date is required")
    private ZonedDateTime date;

    @NotBlank(message = "Content is required")
    private String content;

}
