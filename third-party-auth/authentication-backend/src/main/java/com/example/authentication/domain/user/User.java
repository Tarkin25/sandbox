package com.example.authentication.domain.user;

import com.example.authentication.core.DomainEntity;
import lombok.*;
import lombok.experimental.FieldDefaults;
import org.springframework.data.relational.core.mapping.Table;

@NoArgsConstructor
@AllArgsConstructor
@With
@FieldDefaults(level = AccessLevel.PRIVATE)
@Getter
@Table("users")
public class User extends DomainEntity {

    String googleId;

    String facebookId;

    String email;

    String name;

}
