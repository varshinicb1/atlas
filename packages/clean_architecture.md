---
kind: Package
id: package:clean-architecture
name: Clean Architecture Patterns
version: "1"
purpose: Document clean architecture and hexagonal architecture patterns — dependency inversion, domain-driven design layers, ports and adapters, and use case organization for maintainable software.
problem_solved: Provides a layered architecture that keeps business logic independent of frameworks, databases, and external concerns, enabling isolated testing, technology swaps, and long-term maintainability by enforcing a strict dependency rule (dependencies point inward).
install: No single install — apply the architectural patterns to any project.
dependencies:
  - concept:software-architecture
  - concept:dependency-injection
  - concept:domain-driven-design
concepts:
  - name: Dependency Rule
    id: concept:clean-arch/dependency-rule
    description: "The foundational principle: source code dependencies can only point inward. Outer layers (frameworks, databases, UI) depend on inner layers (business rules), never the reverse. Inner layers define interfaces (ports) that outer layers implement (adapters). This decouples business logic from infrastructure."
  - name: Entities
    id: concept:clean-arch/entities
    description: "Enterprise-wide business objects containing critical business rules. An entity is a plain object or class with methods that enforce invariants. Entities have no dependencies on frameworks, databases, or UI. An entity should be valid in any application context."
  - name: Use Cases
    id: concept:clean-arch/use-cases
    description: "Application-specific business rules that orchestrate entity interactions. A use case (interactor) receives input from an adapter, coordinates entities and services, and returns output. Each use case handles one business flow (CreateOrder, CancelOrder). Use cases depend only on entities and interfaces (ports)."
  - name: Ports
    id: concept:clean-arch/ports
    description: "Interfaces defined by inner layers that outer layers implement. Inbound ports (driving) are use case interfaces that controllers call. Outbound ports (driven) are repository, presenter, and service interfaces that use cases depend on. Ports ensure the dependency rule is enforced at compile time."
  - name: Adapters
    id: concept:clean-arch/adapters
    description: "Implementations of ports. Primary adapters (controllers, CLI, web) drive the application by calling inbound ports. Secondary adapters (database repositories, email senders, HTTP clients) are driven by the application via outbound ports. Adapters contain framework and infrastructure code."
  - name: Hexagonal Architecture (Ports & Adapters)
    id: concept:clean-arch/hexagonal
    description: "Alistair Cockburn's variant — the application core (hexagon) has symmetric ports on all sides. Primary ports receive incoming requests; secondary ports make outgoing requests to external systems. Adapters on each side translate between the core and external technologies. The shape symbolizes equal focus on all integrations."
  - name: Use Case Interactors
    id: concept:clean-arch/interactors
    description: "Object-oriented use cases implementing an input boundary interface. Each interactor has a single method (execute or handle) that takes a request DTO, performs the business logic, and returns a response DTO. Interactors inject dependencies via constructor (ports) and coordinate entity operations."
  - name: DTOs (Data Transfer Objects)
    id: concept:clean-arch/dtos
    description: "Simple data structures that carry data between layers. Request DTOs carry input to use cases. Response DTOs carry output back to presenters. DTOs are serializable and have no behavior. They prevent domain models from leaking into infrastructure layers."
  - name: Repository Pattern
    id: concept:clean-arch/repository
    description: "An outbound port (interface) defining collection-style data access operations: findById, findAll, save, delete. Implementations encapsulate database specifics (SQL, ORM, file system). Use cases depend on the repository interface, not the implementation. Enables swapping databases without changing business logic."
  - name: Presenter
    id: concept:clean-arch/presenter
    description: "An outbound port that formats output for delivery. The use case returns a response DTO to the presenter via its interface. The presenter transforms it into the format needed by the outer layer (HTTP response, CLI output, GUI model). Different presenters can format the same response differently."
  - name: Service Layer
    id: concept:clean-arch/service-layer
    description: "An outer ring that coordinates cross-cutting concerns (transactions, logging, authorization) around use case execution. Services wrap use cases with middleware — they call the use case interface and add cross-cutting behavior before and after. Not to be confused with domain services."
apis:
  - name: Use Case Interface
    id: api:clean-arch/use-case-interface
    signature: "interface CreateOrderUseCase { execute(request: CreateOrderRequest): Promise<CreateOrderResponse> }"
    returns: Interface contract for a use case.
    description: "An inbound port interface that controllers call. Defines a single execute method with typed request and response DTOs. The implementation (interactor) contains business logic. The interface is in the domain layer; the controller is in the adapter layer."
  - name: Repository Interface
    id: api:clean-arch/repository-interface
    signature: "interface OrderRepository { findById(id: OrderId): Promise<Order | null>; save(order: Order): Promise<void>; findByCustomerId(customerId: CustomerId): Promise<Order[]> }"
    returns: Interface contract for data access.
    description: "An outbound port interface in the domain layer. Defines data operations without any framework-specific types. Return types use domain entities. Implementation details (SQL, ORM, in-memory) are in the infrastructure adapter layer."
  - name: Controller Adapter
    id: api:clean-arch/controller
    signature: "class CreateOrderController { constructor(private useCase: CreateOrderUseCase) {} async handle(request: HttpRequest): Promise<HttpResponse> { const response = await this.useCase.execute({ ...request.body }); return { statusCode: 201, body: response } } }"
    returns: An HTTP response.
    description: "A primary adapter that receives HTTP requests, extracts parameters, calls the use case, and returns formatted responses. The controller knows about HTTP but not business logic. It converts request data into the use case's request DTO format."
  - name: Entity factory
    id: api:clean-arch/entity-factory
    signature: "static Order.create(props: CreateOrderProps): Order { if (props.items.length === 0) throw new Error('Order must have items'); return new Order({ id: new OrderId(), ...props, status: 'pending', createdAt: new Date() }); }"
    returns: A new domain entity instance.
    description: "Factory method that enforces domain invariants during entity creation. Validates business rules before construction. The constructor is private — entities are created through named factory methods with descriptive names."
sections:
  - title: Clean Architecture Project Structure
    id: section:clean-arch/project-structure
    content: |
      Organize a TypeScript project following clean architecture:

      ```
      src/
        domain/                  # Inner layer — no external dependencies
          entities/
            Order.ts             # Business entity with invariants
            User.ts
          value-objects/
            OrderId.ts           # Typed IDs, money, email
            Money.ts
          ports/                 # Interfaces (inbound + outbound)
            in/
              CreateOrderUseCase.ts
              CancelOrderUseCase.ts
            out/
              OrderRepository.ts
              PaymentService.ts
          services/
            PricingService.ts    # Domain services with business logic

        application/             # Use cases (interactors)
          orders/
            CreateOrderInteractor.ts
            CancelOrderInteractor.ts

        adapters/                # Outer layer — framework specific
          primary/
            web/
              controllers/
                OrderController.ts
              middleware/
                AuthMiddleware.ts
            cli/
              OrderCommand.ts
          secondary/
            database/
              PrismaOrderRepository.ts
            payment/
              StripePaymentService.ts
            email/
              SendGridEmailService.ts

        infrastructure/
          config/
            DatabaseConfig.ts
          di/
            container.ts         # Dependency injection setup
          main.ts                # Composition root
      ```
  - title: Dependency Injection Setup
    id: section:clean-arch/di
    content: |
      Wiring the layers together at the composition root:

      ```typescript
      import { PrismaClient } from '@prisma/client';
      import { PrismaOrderRepository } from './adapters/secondary/database/PrismaOrderRepository';
      import { StripePaymentService } from './adapters/secondary/payment/StripePaymentService';
      import { CreateOrderInteractor } from './application/orders/CreateOrderInteractor';
      import { OrderController } from './adapters/primary/web/controllers/OrderController';

      // Composition root — wire dependencies
      const prisma = new PrismaClient();
      const orderRepository = new PrismaOrderRepository(prisma);
      const paymentService = new StripePaymentService(process.env.STRIPE_KEY);
      const createOrderUseCase = new CreateOrderInteractor(orderRepository, paymentService);
      const orderController = new OrderController(createOrderUseCase);

      // Express route binds adapter
      app.post('/orders', (req, res) => orderController.handle(req, res));
      ```

      The domain layer has zero imports from frameworks. Use cases import only domain interfaces and DTOs. Adapters import frameworks but never business logic. The dependency inversion ensures testability — mock ports, not implementations.
---
