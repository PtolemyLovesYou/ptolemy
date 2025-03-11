import { gql } from '@apollo/client';

export const GET_USER_PROFILE = gql`
  query Me {
    me {
      id
      username
      displayName
      isAdmin
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
