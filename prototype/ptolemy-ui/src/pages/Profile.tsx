import { useQuery, gql } from '@apollo/client';
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"

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

    const { me } = data;
    const mapToInput = ([key, value]: [string, unknown]) => {
        if (key === '__typename') return null;
        return (
            <div>
                <Label htmlFor={key.toLowerCase()}>{key}</Label>
                <Input type="text" id={key.toLowerCase()} placeholder="(empty)" value={String(value)} disabled />
            </div>
        )
    }
    return (
        <div className="grid gap-5">
            <h1>Profile</h1>
            {Object.entries(me).map(mapToInput)}

            <h2>API Keys</h2>
            <p>Coming soon...</p>
        </div>
    );
};

export default Profile;
