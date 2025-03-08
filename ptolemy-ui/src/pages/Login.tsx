import { z } from 'zod';
import { useForm } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import {
  Form,
  FormControl,
  FormDescription,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from '@/components/ui/form';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { useAuth } from '@/auth/provider';

const formSchema = z
  .object({
    username: z.string().min(5).max(256),
    password: z.string().min(5).max(256),
  })
  .required();

function Login() {
  const { login } = useAuth();

  const loginForm = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
  });

  // 2. Define a submit handler.
  async function onSubmit(values: z.infer<typeof formSchema>) {
    // Do something with the form values.
    // ✅ This will be type-safe and validated.
    const { username, password } = values;
    await login(username, password);
  }

  return (
    <>
      <h1>Login</h1>
      <Form {...loginForm}>
        <form onSubmit={loginForm.handleSubmit(onSubmit)} className='space-y-8'>
          <FormField
            control={loginForm.control}
            name='username'
            render={({ field }) => (
              <FormItem>
                <FormLabel>Username</FormLabel>
                <FormControl>
                  <Input
                    placeholder='username'
                    autoComplete='username'
                    {...field}
                  />
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />
          <FormField
            control={loginForm.control}
            name='password'
            render={({ field }) => (
              <FormItem>
                <FormLabel>Password</FormLabel>
                <FormControl>
                  <Input
                    placeholder='password'
                    autoComplete='password'
                    type='password'
                    {...field}
                  />
                </FormControl>
                <FormDescription>
                  This is your public display name.
                </FormDescription>
                <FormMessage />
              </FormItem>
            )}
          />
          <Button type='submit'>Login</Button>
        </form>
      </Form>
    </>
  );
}

export default Login;
