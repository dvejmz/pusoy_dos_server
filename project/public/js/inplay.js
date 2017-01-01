console.log('Pusoy Dos:: in play');

Vue.component('player', {
    props: ['player'],
    template: '<li><span :class="player.style" class="name">{{ player.name }}</span><span v-if="player.next">*</span></li>'
});
/*
                    {% if player.id == user_id %}
                        <strong>{{player.name}}</strong>
                    {% else %}
                        {{ player.name }}
                    {% endif %} */

var app = new Vue({
    el: "#inplay",
    data: {
        playerList: [
            { name: "Ben Brunton", next: true, style: "logged-in-player" },
            { name: "Testy McTestface", next: false, style: "" }
        ]
    }
}); 
