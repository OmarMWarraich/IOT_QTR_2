import React, { Suspense } from 'react';
import PostList from "./PostList";
import createResource from "./resource";

const resource = createResource()


export default function App() {
  return ( 
    <div>
      <h1>Blog Posts</h1>
      <Suspense fallback={<h1>Loading...</h1>}>
      <PostList resource={resource} />
      </Suspense>
    </div>
  )
}


