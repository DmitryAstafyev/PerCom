package posts.model;


import lombok.*;

import java.time.ZonedDateTime;

@Getter
@Setter
@Builder()
@RequiredArgsConstructor
@EqualsAndHashCode(callSuper = false)
public class Post {

    // @formatter:off
    @NonNull private String id;
    @NonNull private String author;
    @NonNull private ZonedDateTime date;
    @NonNull private String content;
    // @formatter:on

}
