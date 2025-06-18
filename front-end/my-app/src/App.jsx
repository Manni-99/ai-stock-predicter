import { useEffect } from 'react';
import { SearchBar } from './components/SearchBar';
import "./App.css"


function App() {
  useEffect(() => {
    fetch("/api/data")
      .then((res) => res.json())
      .then(console.log)
      .catch(console.error);
  }, []);

  return ( 
    <div className="App">
      <div className="search-bar-container">
          <SearchBar />
          <div>SearchResults</div>
      </div>
   </div>
  );
}

export default App;
