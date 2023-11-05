<script>
    import {invoke} from '@tauri-apps/api'
  
  async function authenticate_user() {
    invoke("full_authenticate_user")
   //     .then((result)->[
    //        console.log(result);
    //    ])
     //   .catch((error)=>console.error( error));
  }
</script>

<button on:click={authenticate_user}>Start</button>
