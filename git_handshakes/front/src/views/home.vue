<script setup>
import { ref } from "vue"

const username_github_for_search = ref("")
const user_github = ref([])
async function search_git_user(username) {
    
    const res = await fetch("http://localhost:3000/search_git_user_info/${username_github_for_search}")
    const data = await res.json()

    user_github.value = data
}
</script>

