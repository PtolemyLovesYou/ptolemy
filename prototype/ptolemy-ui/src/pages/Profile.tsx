import React from 'react';
import { useQuery, gql } from '@apollo/client';

const GET_USER_PROFILE = gql`
    query GetUserProfile {
        User {
            id
            username
            displayName
            isAdmin
            workspaces {
              workspaceId
              workspaceName
            }
            avatarUrl
        }
    }
`;

const Profile: React.FC = () => {
    const { loading, error, data } = useQuery(GET_USER_PROFILE);

    if (loading) return <p>Loading...</p>;
    if (error) return <p>Error: {error.message}</p>;

    const { userProfile } = data;

    return (
        <div>
            <h1>User Profile</h1>
            <img src={userProfile.avatarUrl} alt={`${userProfile.name}'s avatar`} />
            <p>Name: {userProfile.name}</p>
            <p>Email: {userProfile.email}</p>
        </div>
    );
};

export default Profile;
