// Generated script for workflow de99949f-9ad3-4e79-9156-e7c98ad66702
// Generated at 2025-08-31T13:06:43.786Z

import { Stagehand, type ConstructorParams } from '@browserbasehq/stagehand';
import { z } from 'zod';

// Stagehand configuration
const stagehandConfig = (): ConstructorParams => {
    return {
        env: 'BROWSERBASE',
        verbose: 1,
        modelName: 'google/gemini-2.5-flash-preview-05-20',
        modelClientOptions: {
            apiKey: process.env.GOOGLE_API_KEY,
        },
    };
};

async function runWorkflow() {
    let stagehand: Stagehand | null = null;

    try {
        // Initialize Stagehand
        console.log('Initializing Stagehand...');
        stagehand = new Stagehand(stagehandConfig());
        await stagehand.init();
        console.log('Stagehand initialized successfully.');

        // Get the page instance
        const page = stagehand.page;
        if (!page) {
            throw new Error('Failed to get page instance from Stagehand');
        }

        const variables = {
            input_de9994_1: 'Nintendo Switch OLED console',
        };

        // Step 1: Navigate to URL
        console.log('Navigating to: https://amazon.com/');
        await page.goto('https://amazon.com/');

        // Step 2: Perform action
        console.log(
            `Performing action: type ${variables.input_de9994_1} into the search box`,
        );
        await page.act({
            description: `type ${variables.input_de9994_1} into the search box`,
        });

        // Step 3: Perform action
        console.log(`Performing action: click the search button`);
        await page.act({
            description: `click the search button`,
        });

        // Step 4: Extract data
        console.log(
            `Extracting: extract all Nintendo Switch OLED console products visible on this page with their names, prices, and availability status`,
        );
        const extractedData4 = await page.extract({
            instruction: `extract all Nintendo Switch OLED console products visible on this page with their names, prices, and availability status`,
            schema: z.object({
                products: z.array(
                    z.object({
                        name: z.string().optional(),
                        price: z.number().optional(),
                        availability: z.string().optional(),
                        condition: z.string().optional(),
                    }),
                ),
            }),
        });
        console.log('Extracted:', extractedData4);

        // Step 5: Perform action
        console.log(
            `Performing action: click the Nintendo Switch – OLED Model w/White Joy-Con product`,
        );
        await page.act({
            description: `click the Nintendo Switch – OLED Model w/White Joy-Con product`,
        });

        // Step 6: Perform action
        console.log(`Performing action: click the Add to Cart button`);
        await page.act({
            description: `click the Add to Cart button`,
        });

        // Step 7: Perform action
        console.log(`Performing action: click the Cart button`);
        await page.act({
            description: `click the Cart button`,
        });

        // Step 8: Perform action
        console.log(`Performing action: click the Proceed to checkout button`);
        await page.act({
            description: `click the Proceed to checkout button`,
        });

        console.log('Workflow completed successfully');
        return { success: true };
    } catch (error) {
        console.error('Workflow failed:', error);
        return { success: false, error };
    } finally {
        // Clean up
        if (stagehand) {
            console.log('Closing Stagehand connection.');
            try {
                await stagehand.close();
            } catch (err) {
                console.error('Error closing Stagehand:', err);
            }
        }
    }
}

// Single execution
runWorkflow().then((result) => {
    console.log('Execution result:', result);
    process.exit(result.success ? 0 : 1);
});

export default runWorkflow;
