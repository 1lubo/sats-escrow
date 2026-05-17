<script>
  let status = 'Created';

  const nextStep = () => {
    if (status === 'Created') {
      status = 'Funded';
    } else if (status === 'Funded') {
      status = 'AwaitingDelivery';
    } else if (status === 'AwaitingDelivery') {
      status = 'Released';
    }
  };
</script>

<div>
  <h2>Escrow Status: {status}</h2>
  {#if status !== 'Released'}
    <button on:click={nextStep}>Next Step</button>
  {/if}
</div>