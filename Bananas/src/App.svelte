<script>
  import Icon from '@iconify/svelte';
  import CloseIcon from '~icons/mdi/close-outline';
  import MenuIcon from '~icons/material-symbols/menu';
  import DarkModeIcon from '~icons/material-symbols/dark-mode-outline-rounded';
  
  let files = [
    { name: "Document.pdf", size: "1.5 MB", icon: 'mdi:file-document-outline' },
    { name: "Photo.jpg", size: "2.4 MB", icon: 'mdi:image' },
    { name: "Music.mp3", size: "5.6 MB", icon: 'mdi:music' }
  ];

  let directories = [
    { name: "Projects", icon: 'mdi:folder', children: ["Project 1", "Project 2"] },
    { name: "Documents", icon: 'mdi:folder', children: ["Report.docx", "Summary.pdf"] },
    { name: "Media", icon: 'mdi:folder', children: ["Photo1.jpg", "Song.mp3"] }
  ];

  let sidebarVisible = true;
  let darkMode = false;

  function toggleSidebar() {
    sidebarVisible = !sidebarVisible;
  }

  function toggleDarkMode() {
    darkMode = !darkMode;
    document.body.classList.toggle('dark-mode', darkMode);
  }

  let searchText = '';

  function searchFiles() {
    return files.filter(file => file.name.toLowerCase().includes(searchText.toLowerCase()));
  }
</script>

<style>
  :global(body.dark-mode) {
    background-color: #1e1e1e;
    color: #ccc;
  }
  main {
    display: flex;
    font-family: 'Roboto', sans-serif;
    height: 100vh;
  }
  .sidebar {
    width: 250px;
    background-color: #fafafa;
    padding: 20px;
    box-shadow: -2px 0 5px rgba(0,0,0,0.1);
    overflow-y: auto;
    transition: transform 0.3s;
    height: 100%;
    z-index: 2;
    border-right: 1px solid #e0e0e0;
    position: relative;
  }
  .sidebar.hide {
    transform: translateX(-120%);
  }
  .content {
    flex-grow: 1;
    overflow-y: auto;
    padding: 16px;
    transition: margin-left 0.3s;
    background-color: #fff;
    margin-left: 0px;
  }
  .content.expanded {
    /* transition right by 100 pixels */
    margin-left: -255px;
  }
  .file, .directory {
    display: flex;
    align-items: center;
    padding: 10px;
    border-radius: 4px;
    margin-bottom: 8px;
    transition: background-color 0.3s;
  }
  .file:hover, .directory:hover {
    background-color: #f0f0f0;
  }
  .file svg, .directory svg {
    width: 20px;
    height: 20px;
    margin-right: 10px;
    color: #0078d4;
  }
  .directory .children {
    padding-left: 20px;
  }
  .toggle-button{
    position: fixed;
    top: 15px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: none;
    border: none;
    cursor: pointer;
    color: #555;
    z-index: 3;
    padding: 5px;
    border-radius: 50%;
    transition: background-color 0.3s;
  }
  .dark-mode-switch {
    position: fixed;
    bottom: 10px;
    background: none;
    display: flex;
    align-items: center;
    justify-content: center;
    border: none;
    cursor: pointer;
    color: #555;
    z-index: 3;
    padding: 5px;
    border-radius: 50%;
    transition: background-color 0.3s;
  }
  .toggle-button:hover, .dark-mode-switch:hover {
    background-color: #e0e0e0;
  }
  .toggle-button {
    left: 250px;
    transition: left 0.3s;
  }
  .sidebar.hide + .content .toggle-button {
    left: 10px;
  }
  .dark-mode-switch {
    left: 10px;
  }
  input[type="text"] {
    width: 85%;
    padding: 10px;
    border: 1px solid #ccc;
    border-radius: 4px;
    margin-bottom: 16px;
    font-size: 14px;
    transition: border-color 0.3s;
  }
  input[type="text"]:focus {
    border-color: #0078d4;
    outline: none;
  }
</style>

<main>
  <div class={sidebarVisible ? 'sidebar' : 'sidebar hide'}>
    <h2>Directories</h2>
    {#each directories as dir}
      <div class="directory">
        <Icon icon={dir.icon} />
        <div>{dir.name}</div>
      </div>
      <div class="children">
        {#each dir.children as child}
          <div class="file child">
            <Icon icon="mdi:file-document-outline" />
            <div>{child}</div>
          </div>
        {/each}
      </div>
    {/each}
  </div>
  <div class={sidebarVisible ? 'content' : 'content expanded'}>
    <button class="toggle-button" on:click={toggleSidebar}>
      {#if sidebarVisible}
        <CloseIcon />
      {:else}
        <MenuIcon />
      {/if}
    </button>
    <button class="dark-mode-switch" on:click={toggleDarkMode}>
      <DarkModeIcon />
    </button>
    <input type="text" placeholder="Search files..." bind:value={searchText} on:input={searchFiles}>
    {#each searchFiles() as file}
      <div class="file">
        <Icon icon={file.icon} />
        <div>{file.name}</div>
        <div style="margin-left: auto;">{file.size}</div>
      </div>
    {/each}
  </div>
</main>
