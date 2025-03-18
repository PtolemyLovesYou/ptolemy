import { gql } from '@apollo/client';

export const GET_USER_PROFILE = gql`
  query Me {
    me {
      id
      username
      displayName
      isAdmin
      isSysadmin
      status
      workspaces {
        id
        name
      }
      userApiKeys {
        id
        name
        keyPreview
        expiresAt
      }
    }
  }
`;

export const UPDATE_USER = gql`
fragment ReturnsUser on User {
  displayName
  id
  isAdmin
  isSysadmin
  status
  username
}
mutation UpdateUser($userId: UUID!, $displayName: String, $status: UserStatusEnum, $isAdmin: Boolean) {
  user {
    update(userId: $userId, data: {
      displayName: $displayName,
      status: $status,
      isAdmin: $isAdmin
    }) {
      error {
        field
        message
      }
      success
      user {
        ...ReturnsUser
      }
    }
  }
}
`;

export const CREATE_USER_API_KEY = gql`
  mutation CreateUserApiKey($name: String!, $durationDays: Int) {
    user {
      createUserApiKey(name: $name, durationDays: $durationDays) {
        success
        apiKey {
          apiKey
        }
        error {
          field
          message
        }
      }
    }
  }
`;


