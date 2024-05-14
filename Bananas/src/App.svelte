<script>
  import Icon from '@iconify/svelte';
  import CloseIcon from '~icons/mdi/close-outline';
  import MenuIcon from '~icons/material-symbols/menu';
  import DarkModeIcon from '~icons/material-symbols/dark-mode-outline-rounded';
  let files = [
    { name: "Document.pdf", size: "1.5 MB", icon: 'file-document-outline' },
    { name: "Photo.jpg", size: "2.4 MB", icon: 'image' },
    { name: "Music.mp3", size: "5.6 MB", icon: 'music' }
  ];

  let directories = [
    { name: "Projects", icon: 'folder', children: ["Project 1", "Project 2"] },
    { name: "Documents", icon: 'folder', children: ["Report.docx", "Summary.pdf"] },
    { name: "Media", icon: 'folder', children: ["Photo1.jpg", "Song.mp3"] }
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
    background-color: #333;
    color: #fff;
  }
  main {
    display: flex;
    font-family: Arial, sans-serif;
    height: 100vh;
  }
  .sidebar {
    width: 200px;
    background-color: #f0f0f0;
    padding: 20px;
    box-shadow: -2px 0 5px rgba(0,0,0,0.1);
    overflow-y: auto;
    transition: transform 0.3s;
    height: 100%;
    z-index: 2;
  }
  .sidebar.hide {
    transform: translateX(-100%);
  }
  .content {
    flex-grow: 1;
    overflow-y: auto;
    padding: 16px;
    transition: margin-left 0.3s;
  }
  .content.expanded {
    margin-left: 0;
  }
  .file, .directory {
    display: flex;
    align-items: center;
    padding: 8px;
    border-bottom: 1px solid #ccc;
  }
  .file svg, .directory svg {
    width: 16px;
    height: 16px;
  }
  .toggle-button, .dark-mode-switch {
    position: fixed;
    top: 10px;
    background: inherit;
    border: none;
    cursor: pointer;
    color: #555;
  }
  .toggle-button {
    left: 10px;
    z-index: 3;
  }
  .dark-mode-switch {
    right: 50px;
  }
</style>
<main>
  <button class="toggle-button" on:click={toggleSidebar}>
    <MenuIcon /> 

  </button>
  <button class="dark-mode-switch" on:click={toggleDarkMode}>
    <DarkModeIcon />
  </button>
  <div class={sidebarVisible ? 'sidebar' : 'sidebar hide'}>
    <h2>Directories</h2>
    {#each directories as dir}
      <div class="directory">
        <Icon icon={`@iconify-icons/mdi/${dir.icon}`} />
        <div>{dir.name}</div>
      </div>
      {#each dir.children as child}
        <div class="file child">
          <Icon icon={`@iconify-icons/mdi/file-document-outline`} />
          <div>{child}</div>
        </div>
      {/each}
    {/each}
  </div>
  <div class={sidebarVisible ? 'content' : 'content expanded'}>
    <input type="text" placeholder="Search files..." bind:value={searchText} on:input={searchFiles}>
    {#each searchFiles() as file}
      <div class="file">
        <Icon icon={`@iconify-icons/mdi/${file.icon}`} />
        <div>{file.name}</div>
        <div style="margin-left: auto;">{file.size}</div>
      </div>
    {/each}
  </div>
</main>