import { useQuery, gql } from '@apollo/client';

const GET_USER_PROFILE = gql`
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
            <p>Name: {userProfile.displayName}</p>
            <p>Username: {userProfile.username}</p>
        </div>
    );
};

export default Profile;
