<script lang="ts">
    let a = $state("ff");
    let b = $state("ff");
    let c = $state("ff");
    let d = $state("ff");
    let e = $state("ff");
    let f = $state("ff");
    
    let aEl: HTMLInputElement;
    
    let invalid = $state(false);
    
    function change(event: any) {
        invalid = aEl.validity.valid;
    }
</script>

<div class="mb-3">
    <label for="exampleInputPassword1" class="form-label">MAC</label>
    <div class="hstack gap-2">
        <input onchange={change} bind:this={aEl} bind:value={a} pattern="([\da-fA-F]{2})" size="2" type="text" class="form-control" aria-label="MAC a" required>
        <p class="fs-5 mb-1">:</p>
        <input onchange={change} bind:value={b} pattern="([\da-fA-F]{2})" size="2" type="text" class="form-control" aria-label="MAC b" required>
        <p class="fs-5 mb-1">:</p>
        <input onchange={change} bind:value={c} pattern="([\da-fA-F]{2})" size="2" type="text" class="form-control" aria-label="MAC c" required>
        <p class="fs-5 mb-1">:</p>
        <input onchange={change} bind:value={d} pattern="([\da-fA-F]{2})" size="2" type="text" class="form-control" aria-label="MAC d" required>
        <p class="fs-5 mb-1">:</p>
        <input onchange={change} bind:value={e} pattern="([\da-fA-F]{2})" size="2" type="text" class="form-control" aria-label="MAC e" required>
        <p class="fs-5 mb-1">:</p>
        <input onchange={change} bind:value={f} pattern="([\da-fA-F]{2})" size="2" type="text" class="form-control" aria-label="MAC f" required>
    </div>
    {#if invalid}
    <p class="text-danger">Invalid MAC address.</p>
    {/if}
</div>