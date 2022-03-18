# Programação Concorrente - Lista 1
Neste respositório estão presentes as respostas da lista 1 da disciplina Programação Concorrente - UFCG.
A questão 1 está descrita logo abaixo, as questões 2 e 3 foram codificadas em Rust e comentários a respeito estarão presentes nas seções correspondentes.
Abaixo estão as respostas da lista.

### Questão 1:
Conforme a imagem, existem 6 estados que a Thread pode assumir em Java, são elas:
<ol>
  <li>New</li>
  <li>Runnable</li>
  <li>Terminated</li>
  <li>Blocked</li>
  <li>Waiting</li>
  <li>Timed_Waiting</li>
</ol>
<br/>

<ol>
  <li><b>New:</b>
  É o estado inicial que a Thread assume assim que ela é criada. Assim que a thread for inicializada, ela transitará para o estado Runnable. </li>

  <li><b>Runnable:</b> <br/>Thread que está pronta para ser executada, uma thread irá para este estado após sua inicialização (realizada na transição do estado New) ser efetuada. Este estado também comporta as threads que saíram do estado Waiting, Timed_Waiting e Blocked.
O estado runnable não indica necessariamente que a thread está em execução, ela pode
estar esperando que outros recursos sejam disponibilizados, como o tempo de CPU por exemplo.</li>
  
  <li><b>Terminated:</b>
    Este é o estado final do ciclo de vida de uma thread, ele é atingido quando o código termina a execução ou uma exceção é lançada, e assim a thread é finalizada.</li>
  
  <li><b>Blocked:</b>
  Estado ocorre quando uma thread com o estado Runnable tenta adquirir um lock, caso a lock seja adquirida, então a thread retornará ao estado Runnable. Caso contrário, a thread continuará aqui.</li>

  <li><b>Waiting:</b>
  A thread virá para este estado quando executar o método Object.wait sem timeout (esperando um notify para que retorne ao estado Runnable) ou na execução de um método Thread.join sem timeout (em que aguardará por outras threads até que o join possa ser realizado).</li>

  <li><b>Timed_Waiting:</b>
  Estado similar a waiting, com exceção de que os métodos Object.wait e Thread.join possuem timeout. Nestes dois casos, caso a thread não seja notificada ou tenha sido realizado o join, então ela irá retornar ao estado Runnable assim que o tempo definido pelo timeout expirar.
  Além disto, uma thread pode transitar também transitar para este estado caso o método Thread.sleep seja invocado, retornando a thread ao estado Runnable assim que ela tenha dormido pelo tempo determinado.
  </li>
</ol>


### Para as Questões 2 e 3
As questões práticas foram realizadas em Rust em um ambiente Windows 10.
A versão do compilador rustc é 1.57.0 (f1edd0429 2021-11-29).
As demais dependências utilizadas estão disponibilizadas no arquivos <i>cargo.toml</i> correspondentes. 
### Questão 2
Esta questão foi realizada em dois formatos, o primeiro utilizando join no projeto <i>fork-sleep-join</i> e o segundo utilizando semáforos no projeto <i>fork-sleep-join-semaphore</i>.
Comentários relacionados a ela estão presentes na documentação do código.

#### Questão 3
Esta questão está disponível no projeto <i>two-phase-sleep</i>.
Comentários relacionados a ela estão presentes na documentação do código.
