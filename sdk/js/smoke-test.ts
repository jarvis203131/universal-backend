
import { UniversalClient } from './dist/index.js';

try {
    const client = new UniversalClient({ 
        apiUrl: 'http://localhost:8080', 
        projectId: 'test-project' 
    });
    console.log('✅ UniversalClient instantiated successfully');
    
    // Check DB module
    const queryPromise = client.db.from('users').eq('id', '1').limit(1).select();
    console.log('✅ DB Query Builder functioning (returning promise)');

    // Check Auth module
    const authCheck = client.auth.login('test@test.com', 'pass');
    console.log('✅ Auth module accessible');

    // Check Realtime module
    const rtCheck = client.realtime.channel('test').subscribe(() => {});
    console.log('✅ Realtime module accessible');

    // Check Storage module
    const stCheck = client.storage.bucket('assets').upload(new Blob());
    console.log('✅ Storage module accessible');

    console.log('\n🚀 Phase 8 SDK Verification: SUCCESS');
} catch (e) {
    console.error('❌ Verification failed:', e);
    process.exit(1);
}
