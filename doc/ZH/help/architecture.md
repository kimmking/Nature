# Nature 架构

在阅读之前请先阅读[README](../../../README.md)，以了解一些概念和术语。

## Nature 的自然观

Nature 是为了**简化业务系统的建设**而提出的，既要使用简单，又要能够承载复杂的业务。这其实是蛮难的，为此 Nature 借鉴了一下中国古人《易经》的智慧，《易传》有云：“易有太极，是生两仪，两仪生四象，四象生八卦”，这正好满足 Nature 的核心诉求。

之所以将本系统命名为 Nature 也是希望 Nature 能够使用简单的规则来处理广泛而复杂的业务问题。

### 传统系统的问题

#### 单体系统的问题

一般情况下将我们的思想转化为代码需要经过很多的步骤，如：需求调研、设计、编码、测试、部署等，只有这样代码才能“真正懂得”我们的需求，Nature 称之为**决策固化**，也就是说**系统是决策和执行的混合体**。所以研发**周期都比较长**而且，而且经常要迭代，消耗较多的资源。

可悲的是，这个混合体参杂了太多的技术元素，使得决策者并不能清晰的了解决策是否被完美的执行，只有技术人员才知道自己真正做了什么。这是传统开发方式所**固有**的问题，其问题的根源在于**定制**。标准化在纯技术领域表现的非常好，各种开源组件可以说明这个问题。但对于业务领域，在同质化竞争的时代，每个服务提供商都在细节上和对手比拼，这些细节的决策差异必然导致定制，所以没有标准化可言。

#### 多体系统的问题

当业务比较复杂时，我们一般会用多个系统来承载业务。系统大多是按业务的**垂直领域**进行划分的，拿电商来举例，大体上可以分为交易、库房、配送、售后、财务、用户等系统。这样的划分大多与组织机构的划分保持**映射关系**，具有很好的直观性，也是业界的普遍常识。然而系统规模越大系统间的协作成本越高。这是由决策的**联动性**造成的，尤其是核心链路上，系统的迭代会变得**异常艰难**。

导致问题的根本原因依然是决策固化，决策的改变不可避免的或多或少的受现有系统的约束，这表现为相关系统**不能同步上线**，因此我们需要考虑系统**平滑过渡**和**兼容性**问题，这就形成了**难以去除的阶段性代码**（程序员很**善于做加法**而不善于做减法），这无疑会大大增加系统的复杂性。经过长期的迭代后系统就会变得**臃肿**和**难以维护**，所以几百几千人的开发团队就不足为奇了。虽然我们可以通过良好的设计来规避一些问题，但对于初期快速发展的业务来讲很难预判哪个是好的设计，**稍纵即逝的商机**也很难让我们一开始就有一个完美的设计，所以这不太现实。

### 解决问题的方法

要解决这个问题，我们需要先解决定制化的问题，为此 Nature 提供了  [Meta](meta.md) 和 [Relation](relation.md) 以此来解决决策的标准化问题，`Meta`和`Relation` 的定义是不需要编码的，都是机器能够识别的数据，否则决策就是另外一种形式的固化了。标准化有了决策就可以从代码中解脱出来，但解脱出来的决策信息需要有个新的独立于业务系统的新的载体，这就是 Nature。

对于决策来讲最重要的是我们要达成什么样的目标，目标就是数据，因为系统的产出物就是数据。这里的数据是泛指，因此 Nature 称之为 `Meta`。有了目标后，我们就需要定义完成这些目标的方法和规则，方法和规则必须需在多个数据上建立  `Relation` ，因为它们既要输入又要输出，否者规则和方法将失去存在的意义。业务在运行之前必须对`Meta`和`Relation`进行定义，因此 `Meta`和 `Relation` 也被称为设计时。

有了`Meta`和`Relation`我们就可以对传统的业务系统做手术了，将所有的决策取出并放在一起，将剩下的放在一起，形成两个独立的集合。如果 Nature 是太极的话，那么这两个集合便是两仪。这样分散的决策就可以形成有机的整体，形成**决策集中化**。这对于多体系统尤其有用，不用考虑彼此的牵绊而导致的各种问题。决策被从业务系统取出后，原有的业务系统将失去灵魂！这将**导致传统开发方式全面彻底的失效**！

规则和方法定好了必须有人来玩，失去灵魂的剩下的那些部分便是这些玩家，Nature 称之为 [Executor](executor.md) ，`Executor` 的实质是完成 `Relation` 中 关联数据的转换，大多数情况下`Executor`必须由您来自行实现，请放心这个实现已经非常简单了，职责单一，因为复杂的业务耦合已经由 `Relation` 来承担了。`Executor` 也被称之为运行时。

Nature的`设计时`对`运行时`拥有**完全支配能力**。`Meta` 用于限定`运行时`生成什么样的`instance`。而 `Instance` 是 `Meta` 在特定时间、特定场景下生成的特定数据。`Meta` 与 `Instance` 的关系相当于编程语言中的 `Class` 和 `Object`。`Relation` 则指定了`运行时`的规则，也就是说Nature 的**`Meta`和`Relation`是整个业务系统的指挥中心**。

`Meta`, `Relation`, `Executor`和 `Instance` 都是抽象、简洁、规范和统一的形式，所以可以承载众多的不同业务诉求。这像不像《易经》中的四象？

## Nature的时空观

上面主要以问题导向来说明 Nature 是如何解决问题的。接下来说明一下 Nature 的设计理念。

决策为执行提供了运行空间以及空间转换的规则。空间是需要支撑的，对于一个几何体来讲，其支撑物就是点、线、面；注意“面”是**隐含**的，点和线画完了，面自然就出来了，面出来了体也就跟着出来了。对于 Nature 来讲 `Meta` 就是点，代表了业务对象；`Relation` 就是线，代表了业务对象之间的关系。注意：Nature “明确”定义了“面”，它**隐含**在具有层次结构的 `Meta#key`里，用于表示业务领域。 所有的领域构成了完整的业务体系。这样 Nature 只需要 `Meta` 和 `Relation` 两个元素就可以描述繁杂的业务实体，以及实体之间的关系，少了不行，多了也没必要，这就是 Nature 的空间观。

我们再来说说 Nature 的时间观，时间表示运转、秩序、不可变更的历史和演进：

- 运转：只有运转起来的结构（空间）才能行施功能，而 `Relation` 则是运转的发动机！因为Relation 是标准的，用于保障运转的机制便可内建于 Nature 之中，如幂等、重试、异常处理等，这意味着**普通的程序员就可以做出可靠、稳定的系统**！
- 秩序：秩序必须集中制定，然后大家共同遵守。`Relation` 定义了 `Executor` 必须执行的业务规则，最重要的就是上下游 `Meta`。这样先后秩序就有了，也就意味着 `Executor` 只做好自己就可以了，使其逻辑大为简化。
- 不可变更的历史：决策的每次变化都会用版本来记录，既表达了对历史的尊重，又可以使得未完成的业务得以继续。 `Instance`本身就代表着历史， **其一旦生成将永不可篡改**，即便是状态数据，其每一次变更都可以进行**回溯**（也是基于版本号）。
- 演进：Nature 打破了决策固化，使得**决策和执行可以各自独立演进**，不仅如此，Nature 让决策本身更简洁、更精确、更规范；让执行职责更单一、更轻量。这些都会让业务的演进更轻松。

## Nature 运行机制的数学表达

Nature 的运行机制可以用一个数学公式来表达 :

```
 y=f(f(f(...f(x)...)))
```

其意义为：每个`Relation`都可以表示成y=f(x)，上游`Relation`的输出可以作为下游`Relation`的输入。其中x,y都是`Meta`，而 f 则是`Executor`，`Executor`可以看做是传统意义上的接口，接口在传统开发方式中具有举足轻重的地位，是功能间协作的桥梁，但`Relation`将接口的重要性弱化了，这样可以有几个好处：

### 去功能化

传统的接口是一种**功能导向**的产物，功能是用来实现目标的，所以**功能是现象级的**，你得经过分析才知道它要干什么，甚至有时候不知道它在干什么，或者为什么这么干。Nature 是面向目标的，目标是一个点，功能是一条线，所以目标要简单明了的多。

对于一个 y=f(x) 来讲目标就是 y 而作为 `Executor` 的 f 只是一个功能占位符，所以我们可以简化为 

```
x -> y
```

也就是 `Relation`， 这样 Nature 将传统系统本末倒置的**功能与目标的从属关系进行了正位**，使其有了“**自然**”的表达。有了这个表达将带来下面几个重要意义：

- 我们可以更专注于目标，并使得目标简练，直观，可视。
- 目标得以实现标准化并摆脱决策固化的掣肘。
- 目标的标准化带动功能接口的标准化，既 `Executor` 的标准化。

### 去中心化

我们上面说过，上游`Relation`的输出可以作为下游`Relation`的输入。这是一种串行的流式处理方式，在运行时可以一个接一个的**自主流转**并生成相应的`Instance`。这期间不需要某一个或某几个点来进行任何的流程控制，也就是说系统的行为是下游自由发挥的，是不受控制的，是去中心化的。

去中心化在一个大型系统里意义是非凡的。主要体现在以下几个方面

- 效率：去中心化，意味着减少了控制逻辑，意味着用更短的时间更少的资源就能完成任务。意味着可以并发和水平扩展能力而不用担心挤独木桥。
- 自适应：在确保完成任务的前提下，因为控制的去除，就有了更多实现方式的可能性，不用担心过多的管制和约束。

### 一对一

`Relation`只允许一个输入对应一个输出，既一对一，这种做法借鉴于关系数据库，除了形式简单外还有以下几方面原因。

- **强制业务间解耦**。一对多，多对一等各种复杂关系必须用一对一来表达，这样控制形式的多样性就消除了，也就容易统一了，这就为不需要编码的基于简单配置的流程设计提供了基础。
- **高阶功能**更容易达成。每个一对一都是简单的，所以我们可以快速的构建出很多的功能体，当它们通过 Nature 聚合在一起时就像《失控》中描述的蜂群、蚁群那样“**涌现**”出一种高阶能力。
- 一对一可以让 Nature 更容易**赋能**。因为`Executor`形式简单、统一，我们就很容易在`Executor`上施加`切面`技术，如并发、幂等、重试等，从而大幅度降低 `Executor` 开发的技术复杂度，使开发者能够更好的将精力聚焦到业务本身上。

## Nature 运转机制的哲学意义

这里我们讲一下**控制**和**选择**的区别：控制是对他人的控制，选择是对自己的选择与他人无关。

### 控制

控制是实现目标所采取的手段和方法。我们的**系统大多是基于控制的**，代码控制一切，无论是框架、设计模式、组件都是一种控制，包括 Nature，这是编程这个行业诞生以来天经地义的一件事情。从这一点来讲系统就是一个功能体，目标是比较隐性的，**是决策固化的一个明证**。

点越多，控制起来越困难，虽然我们可以通过分层来减少控制的复杂度，但却增加了**反馈路径**的长度。这是大型分布式系统需要直面的艰难抉择，参与者越多，控制逻辑越复杂，直至出现**瓶颈**！

**Nature 只对规则进行控制**，并不对业务本身进行控制，而且 Nature 的规则非常的简单，非常的少，这就保证了 Nature 处理的的高效性。Nature 里最重要的一个规则就是 `Relation` ，它是拒绝控制的，`Relation` 要求所有的业务参与者都基于下面要介绍的`选择`来**自发组织流程**。从这个层面来讲 Nature 可极大的压缩业务开发所带来的控制成本。

### 选择

选择与控制相反，它是自下而上的，就像一条河一样，上游是无法控制下游的流向的。`Relation` 的形式决定了它不允许对业务流程进行控制，只允许选择。`Relation`不只是选择上游，他还选择`Executor`，这样就可以实现“功能”随意替换。

因为上游不去控制，下游也没有必要将信息**反馈**到上游，成为真正高效的流，这点对多级反馈尤其有意义。所以选择既提升了性能，又给了下游充分的灵活性，可以方便的决定自已的意图，掌控自己的命运；同时因为没有集中的抉择，就没有瓶颈问题，就可以水平扩展。

### 对业务迭代的影响

`控制`和`选择`对业务流程的迭代处理是完全不同的，**控制要求必须事先做好分配工作**。这里举一个支付和发货的例子：当支付状态为“已支付”则生成出库申请单，如果没有支付完成则不动作。通常情况下我们会在支付逻辑里使用`if`控制语句，这就是控制！现在我们要做业务调整：由支付状态的已支付改为订单状态的已支付。此时我们要修改两个地方的逻辑，一个是支付的逻辑，一个是订单的逻辑；另外需要考虑两个系统的平滑过渡问题，这个可是个高风险，高耗时，且具有副作用的高难度操作。所以你可以看到虽然是一个小小的决策，控制所带来的代价有多高！

我们再来看看`选择`，Nature 用`选择`来**引导流程**而不是控制流程，所以流程控制里没有`if`。就上面的例子来讲，支付完成不主动触发任何动作，而是下游生成出库申请单时触发的，你只需要定义一个“支付状态=已支付->出库申请单”的 `Relation` 就可以了。当决策变为订单状态驱动时，我们只需将 `Relation` 改为：“订单状态=已支付->出库申请单”及修改对应的 `Executor` 即可。由此可以看出`选择`可以做到**按需修改**，这是控制难以企及的。

`选择`和`控制`只是两种意识形态，没有孰优孰劣的问题，对于一个个体而言，要想对其控制，必须对其所控制的资源进行某种形式的**占有**，控制的能力和范围与资源占有程度是紧密相关的。而占有是具有**侵入性**的，而侵入是有成本的，这对大型分布式系统来讲是非常不利的。但选择只要对方存在就可以，不需要侵入，所以不需要额外的成本，既**选择比控制对环境的要求要低很多**。

Nature 是适合于大型业务系统的，这是 Nature 的选择特性所赋予的。因为没有控制，你不会遇到复杂的逻辑问题，诸如分支、循环、跳转等；因为没有控制，系统的演进成本将会非常低廉，业务模块间的链路能方便的重组并容易找到最优路径；因为没有控制，各种各样的业务模块可以自由的试错和扩展，以灵活的方式去匹配业务发展的需要。

### 生态与法则

在一个生态系统里没有一个至高无上的主可以控制所有的一切，每一个物种都在选择中适应，这样才有了生物的**生机勃勃**，这正是目前传统系统所欠缺的。传统系统之所以死气沉沉，低效，就是因为自上而下的控制，类似于公司业务的层层汇报、层层审批，**当控制的链路非常长非常广时，由控制导致的内耗将十分突出**。

虽然这里不强调控制，但万物运转如何保持其秩序呢？那就是法则。没有控制并不代表着没有法则，法则不是控制，控制有着明确的目的性，而法则没有目的性。地球之所以围绕太阳转，不是太阳有意控制地球，而是万有引力法则在发挥作用。自然界的生态系统也是一样的，春夏秋冬、寒暑易节、昼夜轮回维系着自然生态的平衡。

Nature 的法则就是`选择`，就是 `Relation`，Nature 所有的代码几乎都是围绕着这一法则来组织的，为业务的自由选择提供支撑。Nature 为打造一个**业务生态系统**提供了一个非常便利的工具。

### 选择实现的无形控制

在 [Demo](https://github.com/llxxbb/Nature-Demo) 中 涉及到网购和统计相关的示例，这些示例说明了 Nature 如何简化这些业务的实现。在这里不做具体展开，这里只想说明一下选择机制如何有效支撑系统的运行秩序。为了简单起见，所表达的内容可能与Demo中的不完全一致，还请谅解。

上文中我们说到选择是下游对上游的选择，这就揭示了一种思考方式：**逆向思维**，既我们要达到目的需要什么。拿网购来讲，需要从流程的终点来倒推。用户若想拿到商品需要配送员送，交接数据为签收单，于是我们定义第一个`Meta`。然后我们再倒推，配送员需要和库房交接出库单才能拿到商品进行配送，出库单是我们的第二个`Meta`于是我们有了第一个`Relation` ： 出库单->签收单。

以此类推我们可以定义出类似于下面的`Relation`

```
出库单->签收单
订单->出库单
```

这里有个很重要的点需要说一下，从出库单到签收单如何调度的，是自己公司配送还是第三方配送这里并不关心，`Relation`的`Executor`想用哪个就用哪个。既`Relation`只关心结果，并不关心如何做。

这样一个简单但完整的`设计时`便出来了。而这个设计不用写代码就实现了对`运行时`的控制。

出库单->签收单 在`设计时`是一对一的关系，在`运行时`产出的`Instance`也是一对一的,既一张真实的出库单会对应一张真实的签收单。这在 Nature 里是最为常规的控制。其它控制方式都是由这种常规方式支撑的，如接下来要讲的分流。

设计时的分流的形式是不同的下游拥有同样的上游，就像河流分叉一样。如上面的出库单还可以驱动库存状态，`Relation` 如下：

```
出库单->签收单
出库单->库存状态
```

在此种情形下，出库单不需要知道有多少个下游，但Nature 会知道，因此Nature 需要在幕后分别执行每个`Relation`的`Executor`。

注意这里的“出库单->库存状态”，这个在`设计时`是一对一的，但在`运行时`却有可能是多对一的。举例说明一下，假设在`运行时`我们有两个出库单，出库单1包括2个手机，出库单2包括3个手机，但却只能有一个手机的库存状态，所以在这个示例里就有了两个出库单实例分别对应同一个库存状态的不同版本的实例数据。（有关状态数据及状态的并发控制请看下面技术特性小节）

那么有没有`运行时`是一对多的呢？这里有个例子：订单->支付单。这在`设计时`是一对一的，但在`运行时`是可以一对多的。假设用户第一张卡里的钱不够支付订单，这时候就会有多次支付的情形存在了，既一个订单实例对应多笔支付实例。

那么在`设计时`有没有多对一的情形呢？答案是有的，`Relation`定义如下：

```
出库单->库存状态
入库单->库存状态
```

现在我们来看，`设计时`的一对多、多对一，`运行时`的一对多，多对一，可以随意搭配组合，这就在理论层面形成了完整的闭环。这样`Relation`就可以支撑非常复杂的业务。但对于使用者来讲基本上不用关心流程控制问题，使用者只需要做出合理的选择，大自然（Nature）便会在法则（`Relation`）下无形中操控一切。

## 标准化

Nature 打破了面向功能的传统开发模式，其简化的形式在降低成本的同时也为我们带来了标准化的可能：

其中之一是**决策的标准化**，传统方式下决策固化使得决策十分个性，只有定制的系统才能表达。而 Nature 的决策是标准化的数据，我们只需配置，无需编码便可被执行系统理解。然而 Nature 并不局限于形式的标准化，它更大的意义**决策行为的标准化**，相对于传统方式 Nature 可以大幅度减少执行层面的考虑，直面决策的最终产物——数据。

其二就是**执行标准化**，因为业务间的耦合被斩断，各个业务执行单元就非常的独立，这些执行单元统一由 Nature 来调度，这必然统一了接口形式，自然就标准化了。

其三是**数据规范和纯化**。传统模式下，我们可能有数十上百个数据库成百上千的数据表，这些数据表是临时数据、业务定义，业务控制、技术数据、业务数据等的**大杂烩**。但 Nature 只有三个数据表与业务相关：`Meta` 用于业务定义， `Relation` 用于业务控制，`Instance`用于业务数据。这会很大程度上减少不必要的数据存储，减少数据的冗余，当然我们可能需要一个大型的分布式数据库，如 [Tidb](https://pingcap.com/en/) 

## 技术特性

Nature 目的就是让使用者聚焦于业务而非技术，为了到达易用的目的，Nature 就必须封账这些技术复杂性，包括辣手的**数据一致性**问题。在传统系统里，要想保持数据一致性是一个非常具有挑战性的任务，当面对多个系统协作并保持整体一致性时问题会更加困难。我们有很多的工作都会消耗在这上面。导致不一致的原因可能是因为设计不到位，但更多的原因可能在于要实现一致性开发代价非常高。目前还没有见到有统一的、高效的、开箱即用的解决方案面世，这是萌生创作 Nature 的重要原因。下面我们来看一下 Nature 都为我们提供了哪些技术能力：

### 数据的不可变性与幂等

Nature 只能插入数据不能变更数据，`Instance`一旦生成既被永久定格，这就防止变更导致的数据覆盖问题。这一特性使得 Nature 可以被信赖，因为生成的数据不可抵赖，且可以溯源。数据不可变的另外一个重要应用就是幂等。

为了实现幂等性，Nature 提供了以下措施和建议。

- 主键防重：`Instance` 数据表的主键构成为 ID + `Meta` + 状态版本。
- 预分配ID，在调用Nature 之前预先生成一个ID，或许 facebook 的 snowflake ID 生成器算法是一个不错的选择。使用此ID作为`Instance`的ID，这样当出现环境问题时使用相同的ID提交数据到 Nature 就不会存储多条数据了。如果你不提供ID，Nature 会使用哈希算法为你生成一个。

数据不可变要求数据是无状态的，而 Nature 又是支持状态数据的，如何解决这个逻辑相悖的问题？答案是版本。Nature 为状态数据的每一次状态变更都会生成一个新的`Instance`，但这些不同状态的`Instance`拥有相同的ID和`Meta`，只是版本号是不同的。上面提到的主机组装中的主机便是一个状态数据的例子。cpu 会形成一个主机的状态数据，内存也会形成一个主机的状态数据。这两条状态数据拥有相同的ID，但版本号不同。

### 防重机制

Nature 需要面对下面情形所产生的问题：

- 并发冲突
- 环境变化

我们先看第一种情况，Nature 是**事件驱动**的，既然是事件，就无法确定触发的时机，就可能出现并发冲突问题。如出库单和入库单同时操作商品库存，很显然我们只能让一个成功，另一个失败。Nature 內建了版本冲突的控制，无需`Executor`进行干预，除非外部直接输入。其实现机制是这样的，如果下游数据是状态数据，Nature 在调用`Executor`之前先取出下游状态数据并记录版本号，然后Nature 再调用`Executor`，当`Executor`返回状态数据后，Nature 会将之前记录的版本号+1 赋值给新返回的状态数据，当+1版本的数据已经存在时即可识别为冲突。冲突处理是Nature 內建的功能，`Executor`无需关注。

第二种情况也是比较常见的，如网络往往会有不稳定的情况，在此种情况下 Nature 会重试。但这里有一种业务情景的不稳定，如库存此时没有但下一时刻就有了。此时的控制权在 `Executor`， 如还想继续尝试，则返回环境错误，如果不想再次尝试则返回逻辑异常。

### 任务分发与`Instance`

调度的幂等性几乎遍及Nature的所有运行过程，这里我们讲一下任务分发。举一个例子：一个上游有两个下游跟随者，生成第一个下游时失败了，但第二个却成功了；这时候我们做了一个“危险”的操作，把第一个下游和上游的关系删除了；这时Nature正在重试失败的第一个分支，砰！相同的输入不同的输出！所以Nature 必须避免此类事情的发生。Nature 的做法是将关系产生的所有的任务数据都一同打包并一次性落盘，这样当关系改变时，就不会影响到已经生成的任务数据。

但是如果网络很糟的话，Nature 可能会重复生成任务数据，而这也有可能导致不幂等，所以任务数据本身也需要防重设计，防重的依据就是上游`Instance`的ID。

### `Executor`与`Instance`

Executor细分有三种：

- 前置Executor：在转换之前可以对上游数据进行编辑，如格式转换等。
- 核心Executor（或称之为转换器）：实现上游 Instance 到下游 Instance 的转换。
- 后置Executor：可对转换后的 Instance 进行编辑。

其实不引入前置、后置Executor也是可以的，完全可以使用多个`Relation`来解决。之所以引入前置、后置`Executor`是基于以下几点考虑的。

- 关系主要说明业务实体间的关系，具有业务语义。而前置、后置Executor一般是技术性处理，如果使其关系化，则由关系所呈现出来的业务图会不纯粹并令人费解。
- 从性能上来讲，前置、后置Executor作为中间结果不会落盘，因此要比`Relation`占用更少的资源。

Nature 是一个平台，它可能面对海量的数据和高并发的情景，在这种场景下最好的选择是使用分布式数据库。因为是分布式数据库，事务可能不被支持，在此种情况下如果`Executor`返回多个`Instance`，Nature 必须一条一条的保存这些数据，而这个过程可能被坏的网络环境打断，被打断的任务会被Nature 重新唤起，既`Executor`重新执行了一次任务，而Nature 不能要求`Executor`本身具有幂等性，于是问题出现了：`Executor`可能返回与上次不同的数据！

与任务分发一样，Nature 使用`任务`来解决这个问题，`任务`的内容包含了所有从`Executor`返回的`Instance`。Nature 在逐条保存`Instance`之前先保存这个任务。这样如果被打断，Nature 只需要从之前任务中取出所有的`Instance`重新保存一下就好了。

### 错误、回调

Nature 为`Executor`定义了两种类型的错误：

- `LogicalError`
- `EnvironmentError`

如果`Executor`遇到一个未定义的错误并且应该中断处理，它就可以返回一个`LogicalError`，接下来Nature 会将这个任务从`task`数据表转移的`task—error`数据表，并且不会尝试重新执行这个任务。

然而有些`Executor`因为执行时间很长，所以无论你重试多少次都无法成功，为此Nature 提供了回调机制来解决这个问题。当遇到这种情况时，`Executor`的实现者需要开启一个独立的线程去执行具体的任务，并立即返回一个异步处理信号及可能返回数据的时间给Nature，Nature 会依据此时间推迟下次重试的时间； 当`Executor`真正完成任务时，`Executor`的实现者需要主动调用Nature `callback`  接口并传入处理结果。

转移到`task——error`数据表中的任务都会记录失败的原因以便于使用者进行检查。

### 重试

Nature 在与`Executor`通信或者进行自身调度时会自动捕捉`EnvironmentError`。针对 `EnvironmentError` Nature 实现了一套机制来多次重试，当所有的重试都失败的时候，任务会从`task`数据表转移的`task—error`数据表。

重试可能会产生重复的`task`和`Instance`。如果检测到重复的  `task` 可以直接中断处理， Nature 有独立的重试模块会继续处理。如果检测到重复的  `Instance` 则需要将原有的 `Instance`取出来替换掉当前的 `Instance`并继续后续处理，而不能像 `task` 那样中断处理，因为如果中断就无法形成后续的 `task`。

### 历史回溯

`Relation`可以构建出一张现在运行的业务网。但具体到某一笔业务，要想给出这笔业务是走的业务网中的哪一条或哪几条线路，对于`Relation`来讲是不合适的。Nature 用`Instrance`的`from`属性来解决这个问题，该属性记录了它的上游`Instance`。这样就可以非常方便的知道该笔业务的来龙去脉了。这对于传统业务系统来讲是件非常困难的事情，如需要在不同的页面来回跳转，非常不直观，也非常难于排查问题，这是因为传统业务系统没有统一的处理模型，要做到像Nature 这样易用的程度，非常困难。

### 批处理

假设我们要统计一下一个火爆的电商网站的单品销售 top, 每次统计可能涉及到千万数据，传统的基于 sql 的统计已经不太现实。对于这个问题 Nature 提供了一套自己的解决方案。Nature 提供了一个专有的 `MetaType::Loop` ，Loop 可以**驱动**一次处理一批数据。有两种处理模式：

- MetaSetting.only_one = false

```
Upstream -> Loop + downstream
Loop -> Loop + downstream
...
Loop -> downstream
```

- MetaSetting.only_one = true

```
Upstream -> Loop
Loop -> Loop
...
Loop -> downstream
```

**注意**：对于 `MetaType::Loop` 来讲 `MetaSetting.only_one`如果设置为 true, Nature 会将要输出的 Instance 视为有状态的，只有这样才能实现结果的叠加，才能完成形如 input + old = new 这种形式的数据处理。但你不能把`MetaType::Loop` 的目标 Meta 设置为有状态的！因为从 Nature 外部来看我们只要一个最终结果而不是中间结果，如果置为状态数据会让人感觉到非常奇怪。为了实现这种效果，Nature会把中间结果作为 last_state 数据并带到下一个批次里处理直到完成为止。

批量的控制来源于 Nature 的一个[内置Executor](built-in.md)：`instance-loader` 后面有这样的示例，请参考：[示例及功能讲解](https://github.com/llxxbb/Nature-Demo)。

### 上下文

上下文可提供额外的控制手段，如可通过上下文来编辑流程控制。上下文的另外一个好处是，使得业务数据更加纯粹，使得控制数据和业务数据完全分离。

上下文分为`系统上下文`和`用户上下文`。用户上下文是用户可以自行定义的，而系统上下文是 Nature 自身定义的。系统上下文在Nature 的功能构建上起到很重要的补充。如`MetaType::Loop` 和 `instance-loader` 的协作就用到了三个系统上下文：

- loop.next：用于控制下一个批次的开始 instance 条件
- loop.task：用于传递批数据的处理规则，因为只有第一个批次可以取得处理规则。
- loop.finished：标记所有批次是否处理完成。

除了这些外，还有用于桥接的系统上下文：`target.id` 和 `target.para`。当有 A->B->C的链路时，C想使用A的ID作为自己的ID,而B没有使用A的ID，这时候就需要B架一个桥了。当B为另一个体系的数据时会有这个问题。请参考：[示例及功能讲解](https://github.com/llxxbb/Nature-Demo)。

还有用于动态参数替换的系统上下文：`para.dynamic`。一般我们在配置 Relation 数据时，都是定义好的固定内容。但有时候我们需要运行时确定一些参数，这时候就需要该上下文了。


## 可扩展性

### 业务的扩展性

`Meta` 可以通过版本技术来实现业务的变迁或发展。

### 技术扩展性（还未实现）

Nature 是面向业务的一个开发平台，并用简单的方式构建业务模型。她使技术和业务能够很好的解耦，这使得很多技术不用受限于具体的业务，同时又可以用统一而简单的方式来强化业务的能力，如监控、权限管理、可视化等。
