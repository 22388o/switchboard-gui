<script>
 export let progress;
 export let flip;
</script>

<div class="progress">
    <div class="progress_fill"

         style="width: {progress}%;
                {flip ? "float: left;" : "float: right;"}
                ">
        <span class="progress_text">
            {progress}%
        </span>
    </div>
</div>

<style>

 .progress {
     position: relative;
     width: 100%;
     height: 30px;
     background-color: #ccc;
     overflow: hidden;
 }

 .negative_fill {
     height: 100%;
     width: 0%;
     float: left;
 }

 .progress_fill {
     position: relative;
     background-color: #333;
     width: 25%;
     height: 100%;
     float: left;
 }

 .progress_text {
     position: absolute;
     top: 50%;
     transform: translateY(-50%);
     left: 5px;
     font: bold 14px 'Quicksand', sans-serif;
     color: #fff;
 }
</style>
