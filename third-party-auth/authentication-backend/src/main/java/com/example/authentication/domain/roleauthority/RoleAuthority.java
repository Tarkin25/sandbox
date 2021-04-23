package com.example.authentication.domain.roleauthority;

import com.example.authentication.core.DomainEntity;
import lombok.*;
import lombok.experimental.FieldDefaults;

@NoArgsConstructor
@AllArgsConstructor
@With
@FieldDefaults(level = AccessLevel.PRIVATE)
@Getter
public class RoleAuthority extends DomainEntity {

    Long roleId;

    Long authorityId;

}
