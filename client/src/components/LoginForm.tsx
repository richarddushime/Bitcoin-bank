import React from "react";

const LoginForm: React.FC = () => {
  const handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
  };

  return (
    <form onSubmit={handleSubmit} className="max-w-sm mx-auto">
      <h2 className="text-2xl font-bold mb-4">Login</h2>
      <div className="mb-4">
        <label htmlFor="email" className="block mb-1">
          Email
        </label>
        <input type="email" id="email" className="w-full border rounded py-2 px-3" />
      </div>
      <div className="mb-4">
        <label htmlFor="password" className="block mb-1">
          Password
        </label>
        <input type="password" id="password" className="w-full border rounded py-2 px-3" />
      </div>
      <button type="submit" className="bg-blue-500 text-white py-2 px-4 rounded hover:bg-blue-600">
        Login
      </button>
    </form>
  );
};

export default LoginForm;
