# ɑ *[Langauge Alpha]*
## 목표
알고리즘 문제를 풀기 전에 구조에 대해 시뮬레이션 해보기 위한 인터프리터 언어

## TO-DO
> Tree
> Queue
> ...

## BNF

&lt;program&gt; -&gt; **start:** &lt;code\&gt;  
&lt;code&gt; -&gt; {&lt;states&gt;};  
&lt;states&gt; -&gt; &lt;print&gt; | &lt;block&gt;  
&lt;print&gt; -&gt; **print(**&lt;expr&gt;**)**  
&lt;block&gt; -&gt; &lt;assign&gt; | &lt;for-loop&gt; | &lt;while-loop&gt; |  
&lt;assign&gt; -&gt; [&lt;type&gt;] &lt;varname&gt; **=** &lt;value&gt;  
&lt;for-loop&gt; -&gt; **for** &lt;varname&gt; **in** &lt;list&gt;**:** &lt;states&gt;  
&lt;while-loop&gt; -&gt; **while** &lt;cond&gt;**:** &lt;states&gt;  
&lt;value&gt; -&gt; [**new** &lt;type&gt;]**(**{&lt;expr&gt;}**);** | &lt;expr&gt;;  
&lt;expr&gt; -&gt; &lt;expr&gt; (* | /) &lt;term&gt;  
&lt;term&gt; -&gt; &lt;term&gt; (+ | -) &lt;term&gt;  
*WIP, 잘 기억안나서 수정 필요*    