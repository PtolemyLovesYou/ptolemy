import { useQuery, gql } from '@apollo/client';
import { format } from "date-fns"
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';

import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/components/ui/table';
import { Checkbox } from '@/components/ui/checkbox';
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog';
import { DialogTrigger } from '@radix-ui/react-dialog';
import { Button } from '@/components/ui/button';
import { Form, FormControl, FormDescription, FormField, FormItem, FormLabel, FormMessage } from '@/components/ui/form';
import { useForm } from 'react-hook-form';
import { z } from 'zod';
import { zodResolver } from '@hookform/resolvers/zod';
import { Popover, PopoverContent, PopoverTrigger } from '@/components/ui/popover';
import { Calendar } from '@/components/ui/calendar';
import { CalendarIcon } from 'lucide-react';
import { cn } from '@/lib/utils';

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
  id: string;
  name: string;
  keyPreview: string;
  expiresAt: Date;
}

interface APIKeysProps {
  apiKeys: UserApiKey[];
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
              <TableCell key={j}>
                {row[col as keyof UserApiKey].toString()}
              </TableCell>
            ))}
            <TableCell>
              <Checkbox id={`delete-${row.id}`} />
            </TableCell>
          </TableRow>
        ))}
      </TableBody>
    </Table>
  );
}

const formSchema = z
  .object({
    name: z.string().max(256),
    expiresAt: z.date(), // TODO - logical date limits
  })
  .required();


function CreateAPIKeyForm() {
  const createKeyForm = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
  });

  // 2. Define a submit handler.
  async function onSubmit(values: z.infer<typeof formSchema>) {
    const { name, expiresAt } = values;
    alert(`Name: ${name}, Expires: ${expiresAt}`)
  }
    return (
    <Form {...createKeyForm}>
    <form onSubmit={createKeyForm.handleSubmit(onSubmit)} className='space-y-8'>
      <FormField
        control={createKeyForm.control}
        name='name'
        render={({ field }) => (
          <FormItem>
            <FormLabel>Name</FormLabel>
            <FormControl>
              <Input
                placeholder='name'
                autoComplete='off'
                {...field}
              />
            </FormControl>
            <FormMessage />
          </FormItem>
        )}
      />
      <FormField
        control={createKeyForm.control}
        name='expiresAt'
        render={({ field }) => (
          <FormItem>
            <FormLabel>Expires</FormLabel>
            <Popover>
                <PopoverTrigger asChild>
                  <FormControl>
                    <Button
                      variant={"outline"}
                      className={cn(
                        "w-[240px] pl-3 text-left font-normal",
                        !field.value && "text-muted-foreground"
                      )}
                    >
                      {field.value ? (
                        format(field.value, "PPP")
                      ) : (
                        <span>Pick a date</span>
                      )}
                      <CalendarIcon className="ml-auto h-4 w-4 opacity-50" />
                    </Button>
                  </FormControl>
                </PopoverTrigger>
                <PopoverContent className="w-auto p-0" align="start">
                  <Calendar
                    mode="single"
                    selected={field.value}
                    onSelect={field.onChange}
                    disabled={(date) =>
                      date < new Date()
                    }
                    initialFocus
                  />
                </PopoverContent>
              </Popover>
            <FormDescription>
              When should the API Key expire?
            </FormDescription>
            <FormMessage />
          </FormItem>
        )}
      />
      <Button type='submit'>Create</Button>
    </form>
        </Form>
    )
}

const Profile: React.FC = () => {
  const { loading, error, data } = useQuery(GET_USER_PROFILE);

  if (loading) return <p>Loading...</p>;
  if (error) return <p>Error: {error.message}</p>;

  const {
    me: { userApiKeys, __typename: _, ...profile },
  } = data;
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
      {userApiKeys.length ? (
        <APIKeys apiKeys={userApiKeys} />
      ) : (
        <p>No API Keys available</p>
      )}
      <div>
        <Dialog>
          <DialogTrigger asChild>
            <Button>Create API Key</Button>
          </DialogTrigger>
          <DialogContent>
            <DialogHeader>
              <DialogTitle>Create User API Key</DialogTitle>
              <DialogDescription>
                <CreateAPIKeyForm/>
              </DialogDescription>
            </DialogHeader>
          </DialogContent>
        </Dialog>
      </div>
    </div>
  );
};

export default Profile;
