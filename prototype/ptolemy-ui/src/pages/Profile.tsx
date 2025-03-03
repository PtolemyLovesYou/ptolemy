import { useQuery, gql } from '@apollo/client';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from '@/components/ui/table';
import { Checkbox } from '@/components/ui/checkbox';

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
      userApiKeys {
         id
         name
         keyPreview
         expiresAt
      }
    }
  }
`;

interface UserApiKey {
    id: string
    name: string
    keyPreview: string
    expiresAt: Date
}

interface APIKeysProps {
    apiKeys: UserApiKey[]
}

function APIKeys({ apiKeys }: APIKeysProps) {
    return (
        <Table>
        <TableHeader>
          <TableRow>
                    <TableHead>ID</TableHead>
                    <TableHead>Name</TableHead>
                    <TableHead>Key Preview</TableHead>
                    <TableHead>Expires</TableHead>
                    <TableHead>Delete?</TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          {apiKeys.map((row, i) => (
            <TableRow key={i}>
              {['id', 'name', 'keyPreview', 'expiresAt'].map((col, j) => (
                <TableCell key={j}>{row[col as keyof UserApiKey].toString()}</TableCell>
              ))}
                  <TableCell><Checkbox id={`delete-${row.id}`} /></TableCell>
            </TableRow>
          ))}
        </TableBody>
      </Table>
    )
}

const Profile: React.FC = () => {
  const { loading, error, data } = useQuery(GET_USER_PROFILE);

  if (loading) return <p>Loading...</p>;
  if (error) return <p>Error: {error.message}</p>;

  const { me: { userApiKeys, __typename: _, ...profile } } = data;
  const mapToInput = ([key, value]: [string, unknown]) => {
    return (
      <div key={key} className='grid max-w-sm gap-1.5'>
        <Label htmlFor={key.toLowerCase()}>{key}</Label>
        <Input
          type='text'
          id={key.toLowerCase()}
          placeholder='(empty)'
          value={String(value)}
          disabled
        />
      </div>
    );
  };
  return (
    <div className='grid gap-5'>
      <h1>Profile</h1>
      {Object.entries(profile).map(mapToInput)}

      <h2>API Keys</h2>
        {userApiKeys.length ? <APIKeys apiKeys={userApiKeys}/> : 'No API Keys available'}
    </div>
  );
};

export default Profile;
