import { RaInput } from "@/components/RaInput";
import { Button } from "@/components/ui/button";
import { Form, required, useLogin } from "ra-core";

export const LoginPage = () => {
    const login = useLogin();

	return (
    <div className="min-h-screen flex">
      <div className="container relative hidden flex-col items-center justify-center md:grid lg:max-w-none lg:grid-cols-2 lg:px-0">
        <div className="relative hidden h-full flex-col bg-muted p-10 text-white dark:border-r lg:flex">
          <div className="absolute inset-0 bg-zinc-900" />
          <div className="relative z-20 flex items-center text-lg font-medium">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              strokeWidth="2"
              strokeLinecap="round"
              strokeLinejoin="round"
              className="mr-2 h-6 w-6"
            >
              <path d="M15 6v12a3 3 0 1 0 3-3H6a3 3 0 1 0 3 3V6a3 3 0 1 0-3 3h12a3 3 0 1 0-3-3" />
            </svg>
            Acme Inc
          </div>
          <div className="relative z-20 mt-auto">
            <blockquote className="space-y-2">
              <p className="text-lg">
                &ldquo;React-admin has allowed us to quickly create and evolve a
                powerful tool that otherwise would have taken months of time and
                effort to develop.&rdquo;
              </p>
              <footer className="text-sm">John Doe</footer>
            </blockquote>
          </div>
        </div>
        <div className="lg:p-8">
          <div className="mx-auto flex w-full flex-col justify-center space-y-6 sm:w-[350px]">
            <div className="flex flex-col space-y-2 text-center">
              <h1 className="text-2xl font-semibold tracking-tight">Sign in</h1>
              <p className="text-sm leading-none text-muted-foreground">
                Try janedoe@acme.com / password
              </p>
            </div>
            <Form className="space-y-8" onSubmit={(values:any) => login(values)}>
              <RaInput
                label="Email"
                source="email"
                type="email"
                validate={required()}
              />
              <RaInput
                label="Password"
                source="password"
                type="password"
                validate={required()}
              />
              <Button type="submit">Sign in</Button>
            </Form>
          </div>
        </div>
      </div>
    </div>
  );
};
