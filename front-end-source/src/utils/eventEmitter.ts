type EventHandlerMap = {
  'submitCode': () => void
  'reqGetCode': () => void
  'resGetCode': (code: string) => void
}
type EventNames = keyof EventHandlerMap

class EventEmitter {

  private handlersMapping: Map<EventNames, Set<EventHandlerMap[EventNames]>> = new Map();

  listen<E extends EventNames>(e: E, fn: EventHandlerMap[E]) {
    if (!this.handlersMapping.has(e)) {
      this.handlersMapping.set(e, new Set());
    }
    this.handlersMapping.get(e)!.add(fn);
  }

  emit<E extends EventNames>(e: E, ...args: Parameters<EventHandlerMap[E]>) {
    const handlers = this.handlersMapping.get(e);
    if (handlers) {
      // @ts-ignore
      handlers.forEach(handler => handler(...args));
    }

  }

  off<E extends EventNames>(e: E, fn: EventHandlerMap[E]) {
    const handlers = this.handlersMapping.get(e);
    if (!handlers)
      return
    handlers.delete(fn);
  }
}

export default new EventEmitter()
